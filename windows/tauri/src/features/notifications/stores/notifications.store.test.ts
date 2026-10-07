import { beforeEach, describe, expect, test } from "bun:test";
import { useNotificationsStore } from "./notifications.store";

const actions = () => useNotificationsStore.getState().actions;
const notifications = () => useNotificationsStore.getState().notifications;

describe("notifications store", () => {
  beforeEach(() => {
    actions().clear();
  });

  test("keeps one entry and counts repeated content", () => {
    actions().record({ id: "toast-1", message: "Build failed", type: "error" });
    expect(notifications()).toHaveLength(1);
    expect(notifications()[0]).toMatchObject({ id: "toast-1", message: "Build failed", count: 1 });

    actions().record({ id: "toast-2", message: "Build failed", type: "error" });
    expect(notifications()).toHaveLength(1);
    expect(notifications()[0]).toMatchObject({
      id: "toast-1",
      message: "Build failed",
      count: 2,
      read: false,
    });

    actions().record({ id: "toast-3", message: "Build failed", type: "error" });
    expect(notifications()).toHaveLength(1);
    expect(notifications()[0].count).toBe(3);
  });

  test("re-reporting a visible notification never raises the count", () => {
    // The recorder re-reports every toast that is still on screen whenever the
    // toast list changes, so only a genuinely new appearance may count up.
    actions().record({ id: "toast-1", message: "Build failed", type: "error" });
    actions().record({ id: "toast-2", message: "Build failed", type: "error" });
    expect(notifications()[0].count).toBe(2);

    actions().record({
      id: "toast-1",
      message: "Build failed",
      type: "error",
      isNewOccurrence: false,
    });
    actions().record({
      id: "toast-2",
      message: "Build failed",
      type: "error",
      isNewOccurrence: false,
    });

    expect(notifications()).toHaveLength(1);
    expect(notifications()[0].count).toBe(2);
  });

  test("never merges notifications with different content", () => {
    actions().record({ id: "toast-1", message: "Build failed", type: "error" });
    actions().record({ id: "toast-2", message: "Build failed", type: "warning" });
    actions().record({ id: "toast-3", message: "Build failed", description: "Java", type: "error" });
    actions().record({ id: "toast-4", message: "Tests passed", type: "error" });

    expect(
      notifications().map((item) => [item.type, item.message, item.description, item.count]),
    ).toEqual([
      ["error", "Tests passed", undefined, 1],
      ["error", "Build failed", "Java", 1],
      ["warning", "Build failed", undefined, 1],
      ["error", "Build failed", undefined, 1],
    ]);
  });

  test("updates a source whose message changed without counting it again", () => {
    actions().record({ id: "toast-1", message: "Connecting", type: "info" });
    actions().record({ id: "toast-1", message: "Connected", type: "success" });

    expect(notifications()).toHaveLength(1);
    expect(notifications()[0]).toMatchObject({
      id: "toast-1",
      message: "Connected",
      type: "success",
      count: 1,
    });
  });

  test("a repeated notification returns to the front as unread", () => {
    actions().record({ id: "toast-1", message: "Build failed", type: "error" });
    actions().record({ id: "toast-2", message: "Tests passed", type: "success" });
    actions().markAllRead();
    expect(notifications().every((item) => item.read)).toBe(true);

    actions().record({ id: "toast-3", message: "Build failed", type: "error" });

    expect(notifications().map((item) => item.message)).toEqual(["Build failed", "Tests passed"]);
    expect(notifications()[0]).toMatchObject({ count: 2, read: false });
    expect(notifications()[0].updatedAt).toBeGreaterThanOrEqual(notifications()[0].createdAt);
  });
});
