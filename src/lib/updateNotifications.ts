export type UpdateReminder = "hour" | "day" | "week" | "release";

export interface UpdateNotificationPreference {
  skippedVersion?: string;
  remindAfter?: number;
}

const STORAGE_KEY = "prep-my-windows.update-notification";
const REMINDER_HOURS = { hour: 1, day: 24, week: 168 } as const;

export function loadUpdateNotificationPreference(): UpdateNotificationPreference {
  let value: unknown;
  try {
    value = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "null");
  } catch {
    return {};
  }
  if (!value || typeof value !== "object") return {};
  const preference = value as Record<string, unknown>;
  return {
    ...(typeof preference.skippedVersion === "string"
      ? { skippedVersion: preference.skippedVersion }
      : {}),
    ...(typeof preference.remindAfter === "number" &&
    Number.isSafeInteger(preference.remindAfter) &&
    preference.remindAfter > 0
      ? { remindAfter: preference.remindAfter }
      : {}),
  };
}

export function saveUpdateNotificationPreference(preference: UpdateNotificationPreference) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(preference));
}

export function shouldNotifyForUpdate(
  version: string,
  preference: UpdateNotificationPreference,
  now: number,
) {
  return (
    preference.skippedVersion !== version &&
    (!preference.remindAfter || preference.remindAfter <= now)
  );
}

export function deferUpdateNotification(
  version: string,
  reminder: UpdateReminder,
  now: number,
): UpdateNotificationPreference {
  if (reminder === "release") return { skippedVersion: version };
  return { remindAfter: now + REMINDER_HOURS[reminder] * 60 * 60_000 };
}
