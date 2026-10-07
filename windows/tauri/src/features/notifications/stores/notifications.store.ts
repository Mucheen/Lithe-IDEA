import { create } from "zustand";
import type { NotificationEntry, NotificationType } from "../types/notifications.types";
import { createSelectors } from "@/utils/zustand-selectors";

const MAX_NOTIFICATIONS = 20;

/**
 * A report of one notification source, such as a single toast.
 *
 * `isNewOccurrence` separates "the same source reported itself again" from
 * "the same content appeared again": the recorder re-reports every toast that
 * is still on screen whenever the toast list changes, and only a genuinely new
 * appearance may raise the merged occurrence count.
 */
export interface NotificationReport {
  id: string;
  message: string;
  description?: string;
  type: NotificationType;
  isNewOccurrence?: boolean;
}

interface NotificationsState {
  notifications: NotificationEntry[];
  actions: {
    record: (notification: NotificationReport) => void;
    markAllRead: () => void;
    remove: (id: string) => void;
    clear: () => void;
  };
}

/** Two notifications merge only when all user-visible content matches. */
function hasSameContent(entry: NotificationEntry, report: NotificationReport) {
  return (
    entry.type === report.type &&
    entry.message === report.message &&
    (entry.description ?? "") === (report.description ?? "")
  );
}

/** Moves a merged entry to the front and keeps the retained-entry limit. */
function promote(notifications: NotificationEntry[], entry: NotificationEntry) {
  return [entry, ...notifications.filter((item) => item.id !== entry.id)].slice(0, MAX_NOTIFICATIONS);
}

export const useNotificationsStore = createSelectors(
  create<NotificationsState>()((set) => ({
    notifications: [],
    actions: {
      record: (notification) =>
        set((state) => {
          const { isNewOccurrence, ...reported } = notification;
          const now = Date.now();
          const sameSource = state.notifications.find((item) => item.id === reported.id);

          if (sameSource) {
            // The same source re-reported itself, possibly with new text. That
            // is an update of one notification, never an extra occurrence.
            return {
              notifications: promote(state.notifications, {
                ...sameSource,
                ...reported,
                updatedAt: now,
                read: false,
              }),
            };
          }

          const sameContent = state.notifications.find((item) => hasSameContent(item, reported));

          if (sameContent) {
            // Repeated content keeps the row it already owns and only counts up.
            return {
              notifications: promote(state.notifications, {
                ...sameContent,
                ...reported,
                id: sameContent.id,
                count: isNewOccurrence === false ? sameContent.count : sameContent.count + 1,
                updatedAt: now,
                read: false,
              }),
            };
          }

          return {
            notifications: [
              { ...reported, count: 1, createdAt: now, updatedAt: now, read: false },
              ...state.notifications,
            ].slice(0, MAX_NOTIFICATIONS),
          };
        }),
      markAllRead: () =>
        set((state) => ({
          notifications: state.notifications.map((item) => ({ ...item, read: true })),
        })),
      remove: (id) =>
        set((state) => ({
          notifications: state.notifications.filter((item) => item.id !== id),
        })),
      clear: () => set({ notifications: [] }),
    },
  })),
);
