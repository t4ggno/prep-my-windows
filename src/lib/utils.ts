import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function errorMessage(error: unknown) {
  if (typeof error === "string") return error;
  if (error instanceof Error) return error.message;
  return "The operation failed";
}

export function formatTimestamp(value: string | null) {
  if (!value) return "Never";
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "medium",
  }).format(new Date(value));
}

const relativeUnits: Array<[Intl.RelativeTimeFormatUnit, number]> = [
  ["day", 86_400_000],
  ["hour", 3_600_000],
  ["minute", 60_000],
];

export function formatRelative(value: string | null, now: number) {
  if (!value) return "Never";
  const elapsed = new Date(value).getTime() - now;
  const formatter = new Intl.RelativeTimeFormat(undefined, { numeric: "auto" });
  for (const [unit, size] of relativeUnits) {
    if (Math.abs(elapsed) >= size) return formatter.format(Math.round(elapsed / size), unit);
  }
  return "Just now";
}

export function matchesQuery(query: string, ...values: Array<string | null | undefined>) {
  const normalized = query.trim().toLowerCase();
  return !normalized || values.some((value) => value?.toLowerCase().includes(normalized));
}

export function countBy<T>(items: T[], key: (item: T) => string) {
  const counts = new Map<string, number>();
  for (const item of items) {
    const value = key(item);
    counts.set(value, (counts.get(value) ?? 0) + 1);
  }
  return counts;
}
