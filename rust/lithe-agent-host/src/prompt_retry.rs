//! Bounded recovery of rejected Claude API-key prompts, before any work starts.
//!
//! The native CLI has one retry budget for permanent and temporary errors and
//! may honor minutes of Retry-After. Disable that layer through public options;
//! retry only categorical ACP failures here, retaining one busy turn and owner.

use std::{future::Future, time::Duration};

use agent_client_protocol::{
    schema::v1::{CancelNotification, PromptRequest, PromptResponse, StopReason},
    Agent, ConnectionTo, Error,
};
use tokio::{sync::watch, time::Instant};

use crate::{AgentEvent, Emit};

pub(crate) const MAX_ATTEMPTS: u32 = 5;
/// Total reconnecting window, excluding the first attempt and cancellation ACK.
pub(crate) const RETRY_WINDOW: Duration = Duration::from_secs(20);

#[derive(Clone)]
pub(crate) struct State {
    eligible: bool,
    pub(crate) cancelling: bool,
    deadline: Option<Instant>,
    last_failure: Option<String>,
    provider_message: Option<String>,
}

impl State {
    pub(crate) fn new(enabled: bool) -> Self {
        Self {
            eligible: enabled,
            cancelling: false,
            deadline: None,
            last_failure: None,
            provider_message: None,
        }
    }

    /// Output or a permission request makes replay unsafe and ends the short
    /// retry window. Normal reasoning, tool and permission limits still apply.
    pub(crate) fn progress(&mut self) {
        self.eligible = false;
        self.deadline = None;
    }

    /// AIR's public failure extension suppresses synthetic assistant error text.
    /// Retain its title for the terminal error; categories never come from text.
    pub(crate) fn observe_failure(&mut self, update: &serde_json::Value) {
        let air = &update["_meta"]["jetbrains"]["air"];
        let failure = &air["sessionFailure"];
        if update["sessionUpdate"] == "session_info_update"
            && air["version"] == 1
            && failure["severity"] == "error"
        {
            if let Some(title) = failure["title"]
                .as_str()
                .filter(|title| !title.trim().is_empty() && title.len() <= 8192)
            {
                self.provider_message = Some(failure_message(failure, title));
            }
        }
    }

    pub(crate) fn timeout_message(&self) -> String {
        match &self.last_failure {
            Some(message) if self.deadline.is_some() => {
                format!("Reconnecting exceeded 20 seconds. The turn was stopped. {message}")
            }
            _ => crate::PROMPT_TIMEOUT_MESSAGE.into(),
        }
    }
}

/// Use the adapter's public errorKind convention; never infer retryability from
/// arbitrary human-readable text, HTTP-looking model prose or unknown errors.
fn retryable(error: &Error) -> bool {
    if let Some(failure) = error
        .data
        .as_ref()
        .and_then(|data| data.get("sessionFailure"))
    {
        // Recovery actions are the adapter's explicit policy. Request, access
        // and exhausted-quota failures carry no retry action in this lane.
        // Some gateways' 400/402 bodies become generic service failures in the
        // CLI. Its error banner may veto retries; it can never enable one.
        if reported_http_status(failure["title"].as_str().unwrap_or_default())
            .is_some_and(|status| status < 500 && !matches!(status, 408 | 409 | 429))
        {
            return false;
        }
        return matches!(failure["category"].as_str(), Some("service" | "limit"))
            && failure["actions"]
                .as_array()
                .is_some_and(|actions| actions.iter().any(|action| action == "retry"));
    }
    matches!(
        error
            .data
            .as_ref()
            .and_then(|data| data.get("errorKind"))
            .and_then(|kind| kind.as_str()),
        Some("rate_limit" | "overloaded" | "server_error" | "transport_lost")
    )
}

/// Narrow compatibility guard for the CLI's provider-error banner, only within
/// a negotiated typed failure. Model text and arbitrary prose never reach it.
fn reported_http_status(title: &str) -> Option<u16> {
    let title = title.strip_prefix("API Error: ")?;
    let title = title.strip_prefix("Request rejected (").unwrap_or(title);
    let code = title.get(..3)?;
    if !code.bytes().all(|byte| byte.is_ascii_digit())
        || !matches!(title.as_bytes().get(3), None | Some(b' ' | b')' | b':'))
    {
        return None;
    }
    code.parse()
        .ok()
        .filter(|status| (100..=599).contains(status))
}

fn failure_message(failure: &serde_json::Value, title: &str) -> String {
    match failure["details"]
        .as_str()
        .filter(|details| !details.trim().is_empty() && details.len() <= 8192)
    {
        Some(details) => format!("{title}\n{details}"),
        None => title.into(),
    }
}

/// Preserve actionable error text without exposing the AIR incident object.
pub(crate) fn error_message(error: &Error) -> String {
    // Keep incident ids, revisions and duplicated JSON internal to retry policy.
    if error
        .data
        .as_ref()
        .is_some_and(|data| data.get("sessionFailure").is_some())
    {
        error.message.clone()
    } else {
        error.to_string()
    }
}

/// Negotiated AIR failures complete with end_turn and a typed response payload,
/// not a JSON-RPC rejection. Normalize them before success reaches the product.
fn terminal_failure(response: &PromptResponse) -> Option<Error> {
    let meta = serde_json::to_value(response).ok()?;
    let air = &meta["_meta"]["jetbrains"]["air"];
    let failure = &air["sessionFailure"];
    if air["version"] != 1 || failure["severity"] != "error" {
        return None;
    }
    let title = failure["title"]
        .as_str()
        .filter(|title| !title.trim().is_empty() && title.len() <= 8192)
        .unwrap_or("The Claude request failed.");
    Some(
        Error::new(-32603, failure_message(failure, title))
            .data(serde_json::json!({"sessionFailure": failure})),
    )
}

pub(crate) fn is_progress(update: &serde_json::Value) -> bool {
    match update["sessionUpdate"].as_str() {
        Some("agent_message_chunk" | "agent_thought_chunk") => {
            update["content"]["text"]
                .as_str()
                .is_some_and(|text| !text.is_empty())
                || update["content"]["type"]
                    .as_str()
                    .is_some_and(|kind| kind != "text")
        }
        Some("tool_call" | "tool_call_update" | "plan") => true,
        _ => false,
    }
}

/// Reuse the upstream session and agent, sending another prompt only after the
/// previous one terminated with a known temporary error and no visible work.
pub(crate) async fn run(
    connection: ConnectionTo<Agent>,
    request: PromptRequest,
    state: watch::Sender<State>,
    mut changes: watch::Receiver<State>,
    turn_id: String,
    emit: Emit,
) -> Result<PromptResponse, Error> {
    for attempt in 1..=MAX_ATTEMPTS {
        state.send_modify(|current| current.provider_message = None);
        let mut result = connection.send_request(request.clone()).block_task().await;
        if let Ok(response) = &result {
            if let Some(failure) = terminal_failure(response) {
                result = Err(failure);
            }
        }
        if let (Err(error), Some(message)) = (&mut result, &state.borrow().provider_message) {
            error.message = message.clone();
        }
        let Err(error) = &result else { return result };
        let can_reconnect =
            retryable(error) && state.borrow().eligible && !state.borrow().cancelling;
        if can_reconnect {
            // An ACP failure settled the attempt, but its SDK stream may retain
            // a queued failed request. Clear it even after the final attempt so
            // an explicit next message cannot replay that pending work.
            let _ =
                connection.send_notification(CancelNotification::new(request.session_id.clone()));
        }
        if attempt == MAX_ATTEMPTS || !can_reconnect {
            return result;
        }
        state.send_modify(|current| {
            current
                .deadline
                .get_or_insert_with(|| Instant::now() + RETRY_WINDOW);
            current.last_failure = Some(error_message(error));
        });
        emit(AgentEvent::TurnRetrying {
            session_id: request.session_id.0.to_string(),
            turn_id: turn_id.clone(),
            attempt: attempt + 1,
            max_attempts: MAX_ATTEMPTS,
        });
        // Four delays total 7.5 seconds. Provider Retry-After is not propagated
        // into this interactive policy; the user may retry again after failure.
        let delay = tokio::time::sleep(Duration::from_millis(500 << (attempt - 1)));
        tokio::pin!(delay);
        loop {
            if changes.borrow().cancelling {
                return Ok(PromptResponse::new(StopReason::Cancelled));
            }
            if !changes.borrow().eligible {
                return result;
            }
            tokio::select! {
                _ = &mut delay => break,
                changed = changes.changed() => if changed.is_err() { return result; },
            }
        }
        if state.borrow().cancelling {
            return Ok(PromptResponse::new(StopReason::Cancelled));
        }
        if !state.borrow().eligible {
            return result;
        }
    }
    unreachable!("bounded retry loop always returns its last result")
}

/// Keep the in-flight future alive on timeout so its owner can cancel and await
/// acknowledgment before releasing the turn or stopping the process tree.
pub(crate) async fn wait<F, T>(response: F, mut changes: watch::Receiver<State>) -> Result<T, ()>
where
    F: Future<Output = T>,
{
    let absolute = Instant::now() + crate::PROMPT_TIMEOUT;
    tokio::pin!(response);
    loop {
        let deadline = changes
            .borrow()
            .deadline
            .map_or(absolute, |retry| retry.min(absolute));
        tokio::select! {
            result = &mut response => return Ok(result),
            _ = tokio::time::sleep_until(deadline) => return Err(()),
            changed = changes.changed() => if changed.is_err() { return Err(()); },
        }
    }
}
