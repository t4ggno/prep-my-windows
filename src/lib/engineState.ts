import type { EngineStatus } from "@/types";

export type EngineState = "active" | "busy" | "attention" | "off";

export function engineState(status: EngineStatus, enforcementEnabled: boolean): EngineState {
  if (status.busy) return "busy";
  if (!enforcementEnabled) return "off";
  if (status.lastError) return "attention";
  return "active";
}

export const engineStateLabels: Record<EngineState, string> = {
  active: "Policy active",
  busy: "Enforcing",
  attention: "Needs attention",
  off: "Enforcement off",
};
