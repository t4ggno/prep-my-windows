import {
  Activity,
  AppWindow,
  Ban,
  Boxes,
  Gauge,
  ListRestart,
  LoaderCircle,
  Power,
  PowerOff,
  Settings2,
  ShieldCheck,
} from "lucide-react";
import markUrl from "@/assets/mark.svg";
import { Button } from "@/components/ui/button";
import { useNow } from "@/hooks/useNow";
import { engineState, engineStateLabels } from "@/lib/engineState";
import { cn, formatRelative, formatTimestamp } from "@/lib/utils";
import type { EngineStatus, Section, SystemInfo } from "@/types";

const navigation: Array<{
  id: Section;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
}> = [
  { id: "overview", label: "Overview", icon: Gauge },
  { id: "policy", label: "Windows policy", icon: ShieldCheck },
  { id: "apps", label: "Apps", icon: Boxes },
  { id: "autostart", label: "Autostart", icon: ListRestart },
  { id: "processes", label: "Process rules", icon: Ban },
  { id: "network", label: "Network", icon: AppWindow },
  { id: "activity", label: "Activity", icon: Activity },
  { id: "settings", label: "Settings", icon: Settings2 },
];

export function AppShell({
  section,
  setSection,
  system,
  status,
  enforcementEnabled,
  enforcing,
  switchingEnforcement,
  onEnforce,
  onSetEnforcementEnabled,
  children,
}: {
  section: Section;
  setSection: (section: Section) => void;
  system: SystemInfo;
  status: EngineStatus;
  enforcementEnabled: boolean;
  enforcing: boolean;
  switchingEnforcement: boolean;
  onEnforce: () => void;
  onSetEnforcementEnabled: (enabled: boolean) => void;
  children: React.ReactNode;
}) {
  const now = useNow();
  const state = engineState(status, enforcementEnabled);
  const running = enforcing || status.busy;

  return (
    <div className="app-frame">
      <aside className="sidebar">
        <div className="brand">
          <img className="brand-mark" src={markUrl} alt="" />
          <span>Prep My Windows</span>
        </div>

        <nav className="nav-list">
          {navigation.map((item) => {
            const Icon = item.icon;
            return (
              <button
                key={item.id}
                className={cn("nav-item", section === item.id && "active")}
                aria-current={section === item.id ? "page" : undefined}
                onClick={() => setSection(item.id)}
              >
                <Icon className="size-[17px]" />
                {item.label}
              </button>
            );
          })}
        </nav>

        <button className="sidebar-system" onClick={() => setSection("activity")}>
          <span className="flex items-center gap-2 text-sm text-zinc-300">
            <span className={cn("engine-dot", state)} />
            {engineStateLabels[state]}
          </span>
          <span className="mt-2 block text-xs text-zinc-600">
            Windows 11 {system.displayVersion} · {system.buildNumber}
          </span>
        </button>
      </aside>

      <main className="main-panel">
        <div className="topbar">
          <div className="text-xs text-zinc-500" title={formatTimestamp(status.lastEnforcedAt)}>
            Last enforced {formatRelative(status.lastEnforcedAt, now)}
          </div>
          {enforcementEnabled ? (
            <>
              <Button
                size="sm"
                variant="secondary"
                onClick={() => onSetEnforcementEnabled(false)}
                disabled={switchingEnforcement}
              >
                <PowerOff className="size-4" />
                Disable
              </Button>
              <Button size="sm" onClick={onEnforce} disabled={running}>
                {running ? (
                  <LoaderCircle className="size-4 animate-spin" />
                ) : (
                  <ShieldCheck className="size-4" />
                )}
                {running ? "Enforcing" : "Enforce now"}
              </Button>
            </>
          ) : (
            <Button
              size="sm"
              onClick={() => onSetEnforcementEnabled(true)}
              disabled={switchingEnforcement || running}
            >
              {switchingEnforcement ? (
                <LoaderCircle className="size-4 animate-spin" />
              ) : (
                <Power className="size-4" />
              )}
              Enable
            </Button>
          )}
        </div>
        <div className="content-scroll">{children}</div>
      </main>
    </div>
  );
}
