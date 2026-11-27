import type { PolicyItem, ScheduledTaskItem } from "@/types";

export type RuleFilter = "all" | "on" | "off";
export type StateFilter = "all" | "mismatch" | "matches" | "changed" | "unavailable";

export function valueDiffersFromRecommendation(item: PolicyItem) {
  return item.wanted !== item.recommended;
}

export function ruleDiffersFromRecommendation(item: PolicyItem) {
  return item.enabled !== item.recommendedEnabled;
}

export function policyIsChanged(item: PolicyItem) {
  return valueDiffersFromRecommendation(item) || ruleDiffersFromRecommendation(item);
}

export function policyIsMismatched(item: PolicyItem) {
  return item.available && !item.compliant;
}

export function taskIsChanged(task: ScheduledTaskItem) {
  return !task.enabled;
}

function matchesRule(enabled: boolean, filter: RuleFilter) {
  if (filter === "on") return enabled;
  if (filter === "off") return !enabled;
  return true;
}

export function policyMatches(item: PolicyItem, rule: RuleFilter, state: StateFilter) {
  if (!matchesRule(item.enabled, rule)) return false;
  switch (state) {
    case "all":
      return true;
    case "mismatch":
      return policyIsMismatched(item);
    case "matches":
      return item.available && item.compliant;
    case "changed":
      return policyIsChanged(item);
    case "unavailable":
      return !item.available;
  }
}

export function taskMatches(task: ScheduledTaskItem, rule: RuleFilter, state: StateFilter) {
  if (!matchesRule(task.enabled, rule)) return false;
  switch (state) {
    case "all":
      return true;
    case "mismatch":
      return task.status === "enabled";
    case "matches":
      return task.status === "disabled";
    case "changed":
      return taskIsChanged(task);
    case "unavailable":
      return task.status === "missing";
  }
}
