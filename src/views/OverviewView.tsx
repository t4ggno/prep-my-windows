import {
  AlertTriangle,
  Ban,
  Boxes,
  Check,
  ChevronRight,
  LoaderCircle,
  RotateCcw,
  ShieldCheck,
  ShieldOff,
  X,
} from "lucide-react";
import { StateDot, ViewHeader } from "@/components/common";
import { useNow } from "@/hooks/useNow";
import { engineState, engineStateLabels } from "@/lib/engineState";
import { cn, formatRelative, formatTimestamp } from "@/lib/utils";
import type { Section, Snapshot } from "@/types";

function Metric({
  label,
  value,
  icon: Icon,
  onClick,
}: {
  label: string;
  value: string | number;
  icon: React.ComponentType<{ className?: string }>;
  onClick: () => void;
}) {
  return (
    <button className="metric-card" onClick={onClick}>
      <div className="metric-icon">
        <Icon className="size-[18px]" />
      </div>
      <div className="text-left">
        <div className="metric-value">{value}</div>
        <div className="metric-label">{label}</div>
      </div>
    </button>
  );
}

function SummaryRow({
  label,
  value,
  complete,
  onClick,
}: {
  label: string;
  value: string;
  complete: boolean;
  onClick: () => void;
}) {
  return (
    <button className="summary-row" onClick={onClick}>
      <span className="summary-label">
        <StateDot active={complete} /> {label}
      </span>
      <span className="flex items-center gap-1.5">
        {value}
        <ChevronRight className="size-3.5 text-zinc-700" />
      </span>
    </button>
  );
}

const heroIcons = {
  active: ShieldCheck,
  busy: LoaderCircle,
  attention: AlertTriangle,
  off: ShieldOff,
};

export function OverviewView({
  snapshot,
  onNavigate,
}: {
  snapshot: Snapshot;
  onNavigate: (section: Section) => void;
}) {
  const now = useNow();
  const state = engineState(snapshot.status, snapshot.settings.enforcementEnabled);
  const HeroIcon = heroIcons[state];
  const enabledPolicies = snapshot.policies.filter((item) => item.enabled && item.available);
  const compliantPolicies = enabledPolicies.filter((item) => item.compliant).length;
  const enabledPackages = snapshot.packages.filter((item) => item.enabled);
  const removedPackages = enabledPackages.filter((item) => !item.installed).length;
  const enabledProcesses = snapshot.settings.processRules.filter((item) => item.enabled).length;
  const enabledNetwork = snapshot.networkBlocks.filter((item) => item.enabled);
  const blockedNetwork = enabledNetwork.filter((item) => item.blocked).length;
  const enabledTasks = snapshot.scheduledTasks.filter((item) => item.enabled);
  const disabledTasks = enabledTasks.filter(
    (item) => item.status === "disabled" || item.status === "missing",
  ).length;

  return (
    <div className="view-stack">
      <ViewHeader title="Overview" />

      <section className={cn("hero-status", state)}>
        <div className="hero-shield">
          <HeroIcon className={cn("size-7", state === "busy" && "animate-spin")} />
        </div>
        <div className="min-w-0 flex-1">
          <h2>{engineStateLabels[state]}</h2>
          {state === "attention" && snapshot.status.lastError ? (
            <button
              className="mt-1 block max-w-full truncate text-left text-sm text-amber-200/70 hover:text-amber-100"
              onClick={() => onNavigate("activity")}
            >
              {snapshot.status.lastError}
            </button>
          ) : (
            <div
              className="mt-1 text-sm text-zinc-500"
              title={formatTimestamp(snapshot.status.lastEnforcedAt)}
            >
              Last enforced {formatRelative(snapshot.status.lastEnforcedAt, now)}
            </div>
          )}
        </div>
      </section>

      <section className="metrics-grid">
        <Metric
          label="Settings enforced"
          value={`${compliantPolicies}/${enabledPolicies.length}`}
          icon={Check}
          onClick={() => onNavigate("policy")}
        />
        <Metric
          label="Apps removed"
          value={`${removedPackages}/${enabledPackages.length}`}
          icon={Boxes}
          onClick={() => onNavigate("apps")}
        />
        <Metric
          label="Processes stopped"
          value={snapshot.status.killedTotal}
          icon={Ban}
          onClick={() => onNavigate("processes")}
        />
        <Metric
          label="Changes reapplied"
          value={snapshot.status.repairedTotal}
          icon={RotateCcw}
          onClick={() => onNavigate("activity")}
        />
      </section>

      <div className="overview-columns">
        <section className="panel-card">
          <div className="panel-title">Enforcement</div>
          <div className="summary-list">
            <SummaryRow
              label="Windows policy"
              value={`${compliantPolicies}/${enabledPolicies.length}`}
              complete={compliantPolicies === enabledPolicies.length}
              onClick={() => onNavigate("policy")}
            />
            <SummaryRow
              label="App removal"
              value={`${removedPackages}/${enabledPackages.length}`}
              complete={removedPackages === enabledPackages.length}
              onClick={() => onNavigate("apps")}
            />
            <SummaryRow
              label="Scheduled tasks"
              value={`${disabledTasks}/${enabledTasks.length}`}
              complete={disabledTasks === enabledTasks.length}
              onClick={() => onNavigate("policy")}
            />
            <SummaryRow
              label="Network blocks"
              value={`${blockedNetwork}/${enabledNetwork.length}`}
              complete={blockedNetwork === enabledNetwork.length}
              onClick={() => onNavigate("network")}
            />
            <SummaryRow
              label="Process rules"
              value={`${enabledProcesses} active`}
              complete
              onClick={() => onNavigate("processes")}
            />
          </div>
        </section>

        <section className="panel-card">
          <button className="panel-title panel-link" onClick={() => onNavigate("activity")}>
            Recent activity
            <ChevronRight className="size-3.5" />
          </button>
          <div className="recent-list">
            {snapshot.activity.slice(0, 5).map((event) => (
              <div className="recent-row" key={event.id}>
                <div className={event.success ? "event-icon success" : "event-icon error"}>
                  {event.success ? <Check className="size-3.5" /> : <X className="size-3.5" />}
                </div>
                <div className="min-w-0 flex-1">
                  <div className="truncate text-sm text-zinc-300" title={event.message}>
                    {event.message}
                  </div>
                  <div
                    className="mt-0.5 text-xs text-zinc-600"
                    title={formatTimestamp(event.timestamp)}
                  >
                    {formatRelative(event.timestamp, now)}
                  </div>
                </div>
              </div>
            ))}
            {snapshot.activity.length === 0 && (
              <div className="empty-compact">No activity yet</div>
            )}
          </div>
        </section>
      </div>
    </div>
  );
}
