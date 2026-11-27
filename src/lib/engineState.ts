import type { EngineStatus } from "@/types";

export type EngineState = "active" | "busy" | "off";

export function engineState(status: EngineStatus, enforcementEnabled: boolean): EngineState {
  if (status.busy) return "busy";
  if (!enforcementEnabled) return "off";
  return "active";
}

export const engineStateLabels: Record<EngineState, string> = {
  active: "Policy active",
  busy: "Enforcing",
  off: "Enforcement off",
};
