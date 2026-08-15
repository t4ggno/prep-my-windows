export type Section =
  | "overview"
  | "policy"
  | "apps"
  | "autostart"
  | "processes"
  | "network"
  | "activity"
  | "settings";

export type CatalogSection =
  | "policies"
  | "packages"
  | "networkBlocks"
  | "scheduledTasks";

export interface EngineStatus {
  running: boolean;
  busy: boolean;
  lastEnforcedAt: string | null;
  repairedTotal: number;
  killedTotal: number;
  lastError: string | null;
}

export interface SystemInfo {
  productName: string;
  displayVersion: string;
  buildNumber: string;
  isWindows11: boolean;
  isElevated: boolean;
}

export interface PolicyItem {
  id: string;
  name: string;
  category: string;
  scope: string;
  enabled: boolean;
  compliant: boolean;
  current: string;
  wanted: string;
  available: boolean;
  unavailableReason: string | null;
}

export interface PackageItem {
  id: string;
  name: string;
  packageName: string;
  enabled: boolean;
  installed: boolean;
  provisioned: boolean;
}

export interface InstalledPackage {
  name: string;
  fullName: string;
  publisher: string;
  version: string;
  provisioned: boolean;
  removable: boolean;
}

export interface NetworkItem {
  id: string;
  domain: string;
  enabled: boolean;
  blocked: boolean;
}

export interface ScheduledTaskItem {
  id: string;
  name: string;
  path: string;
  enabled: boolean;
  disabled: boolean;
}

export interface ProcessRule {
  id: string;
  name: string;
  executableName: string;
  executablePath: string | null;
  enabled: boolean;
  builtIn: boolean;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  executablePath: string;
}

export type AutostartKind =
  | "registry"
  | "startupFolder"
  | "scheduledTask"
  | "service";

export interface AutostartEntry {
  id: string;
  name: string;
  command: string;
  kind: AutostartKind;
  source: string;
  location: string;
  state: string;
}

export interface AutostartRule {
  id: string;
  name: string;
  kind: AutostartKind;
  location: string;
}

export interface ActivityEvent {
  id: string;
  timestamp: string;
  kind: string;
  message: string;
  success: boolean;
}

export interface AppSettings {
  policies: Record<string, boolean>;
  packages: Record<string, boolean>;
  networkBlocks: Record<string, boolean>;
  scheduledTasks: Record<string, boolean>;
  processRules: ProcessRule[];
  mutedProcessNotifications: string[];
  customPackages: string[];
  blockedAutostarts: AutostartRule[];
  startWithWindows: boolean;
  activeHoursStart: number;
  activeHoursEnd: number;
}

export interface Snapshot {
  system: SystemInfo;
  status: EngineStatus;
  settings: AppSettings;
  policies: PolicyItem[];
  packages: PackageItem[];
  installedPackages: InstalledPackage[];
  networkBlocks: NetworkItem[];
  scheduledTasks: ScheduledTaskItem[];
  activity: ActivityEvent[];
}

export interface LiveState {
  status: EngineStatus;
  activity: ActivityEvent[];
}

export interface RuntimeSettings {
  startWithWindows: boolean;
  activeHoursStart: number;
  activeHoursEnd: number;
}
