import { invoke } from "@tauri-apps/api/core";
import type {
  AutostartEntry,
  CatalogSection,
  LiveState,
  ProcessInfo,
  RuntimeSettings,
  Snapshot,
} from "@/types";

export const api = {
  snapshot: () => invoke<Snapshot>("get_snapshot"),
  liveState: () => invoke<LiveState>("get_live_state"),
  setCatalogItem: (section: CatalogSection, id: string, enabled: boolean) =>
    invoke<void>("set_catalog_item", { section, id, enabled }),
  setProcessRuleEnabled: (id: string, enabled: boolean) =>
    invoke<void>("set_process_rule_enabled", { id, enabled }),
  addProcessRule: (request: {
    name: string;
    executableName: string;
    executablePath: string | null;
  }) => invoke<void>("add_process_rule", { request }),
  removeProcessRule: (id: string) => invoke<void>("remove_process_rule", { id }),
  listProcesses: () => invoke<ProcessInfo[]>("list_processes"),
  listAutostarts: () => invoke<AutostartEntry[]>("list_autostarts"),
  blockAutostart: (entry: AutostartEntry) =>
    invoke<void>("block_autostart", { entry }),
  removeAutostartRule: (id: string) =>
    invoke<void>("remove_autostart_rule", { id }),
  addCustomPackage: (packageName: string) =>
    invoke<void>("add_custom_package", { packageName }),
  removeCustomPackage: (packageName: string) =>
    invoke<void>("remove_custom_package", { packageName }),
  updateRuntimeSettings: (settings: RuntimeSettings) =>
    invoke<void>("update_runtime_settings", { settings }),
  enforceNow: () => invoke<number>("enforce_now"),
  resetProfile: () => invoke<void>("reset_profile"),
  exportProfile: (path: string) => invoke<void>("export_profile", { path }),
  importProfile: (path: string) => invoke<void>("import_profile", { path }),
  clearActivity: () => invoke<void>("clear_activity"),
};
