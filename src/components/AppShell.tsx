import {
  Activity,
  AppWindow,
  Ban,
  Boxes,
  Gauge,
  ListRestart,
  Settings2,
  ShieldCheck,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn, formatTimestamp } from "@/lib/utils";
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
  enforcing,
  onEnforce,
  children,
}: {
  section: Section;
  setSection: (section: Section) => void;
  system: SystemInfo;
  status: EngineStatus;
  enforcing: boolean;
  onEnforce: () => void;
  children: React.ReactNode;
}) {
  return (
    <div className="app-frame">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark">
            <span />
            <span />
            <span />
            <span />
          </div>
          <span>Prep My Windows</span>
        </div>

        <nav className="nav-list">
          {navigation.map((item) => {
            const Icon = item.icon;
            return (
              <button
                key={item.id}
                className={cn("nav-item", section === item.id && "active")}
                onClick={() => setSection(item.id)}
              >
                <Icon className="size-[17px]" />
                {item.label}
              </button>
            );
          })}
        </nav>

        <div className="sidebar-system">
          <div className="flex items-center gap-2 text-sm text-zinc-300">
            <span className="size-2 rounded-full bg-emerald-400 shadow-[0_0_10px_rgba(52,211,153,.5)]" />
            Policy active
          </div>
          <div className="mt-2 text-xs text-zinc-600">
            Windows 11 {system.displayVersion} · {system.buildNumber}
          </div>
        </div>
      </aside>

      <main className="main-panel">
        <div className="topbar">
          <div className="text-xs text-zinc-500">
            Last enforced {formatTimestamp(status.lastEnforcedAt)}
          </div>
          <Button
            size="sm"
            onClick={onEnforce}
            disabled={enforcing || status.busy}
          >
            <ShieldCheck className={cn("size-4", enforcing && "animate-pulse")} />
            Enforce now
          </Button>
        </div>
        <div className="content-scroll">{children}</div>
      </main>
    </div>
  );
}
