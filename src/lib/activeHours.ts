import type { ActiveHours } from "@/types";

const MAX_ACTIVE_HOURS = 18;

export const HOURS = Array.from({ length: 24 }, (_, hour) => hour);

export function hourLabel(hour: number) {
  return `${hour.toString().padStart(2, "0")}:00`;
}

function span(start: number, end: number) {
  return (end - start + 24) % 24;
}

export function withActiveHoursStart(hours: ActiveHours, start: number): ActiveHours {
  return {
    activeHoursStart: start,
    activeHoursEnd: (start + span(hours.activeHoursStart, hours.activeHoursEnd)) % 24,
  };
}

export function isValidActiveHoursEnd(hours: ActiveHours, end: number) {
  const duration = span(hours.activeHoursStart, end);
  return duration > 0 && duration <= MAX_ACTIVE_HOURS;
}

export function isWithinActiveHours(hours: ActiveHours, hour: number) {
  return span(hours.activeHoursStart, hour) < span(hours.activeHoursStart, hours.activeHoursEnd);
}

export function activeHoursDuration(hours: ActiveHours) {
  return span(hours.activeHoursStart, hours.activeHoursEnd);
}
