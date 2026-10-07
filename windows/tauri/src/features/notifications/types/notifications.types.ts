import type { ReactNode } from "react";

export type NotificationType = "info" | "success" | "warning" | "error";

export interface NotificationEntry {
  id: string;
  message: string;
  description?: string;
  type: NotificationType;
  createdAt: number;
  updatedAt: number;
  read: boolean;
  /**
   * How often the same content has been reported. The first report is 1; later
   * reports raise this count instead of adding a duplicate entry.
   */
  count: number;
}

export interface ToastInput {
  key?: string;
  message: string;
  description?: string;
  type: NotificationType;
  duration?: number;
  icon?: ReactNode;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export type NotificationFilter = "all" | NotificationEntry["type"];

export type NotificationItemAction = {
  id: string;
  label: string;
  icon: ReactNode;
  onSelect: () => void;
  variant?: "default" | "danger";
};
