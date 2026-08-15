import {
  Ban,
  Boxes,
  Check,
  Network,
  RotateCcw,
  ShieldCheck,
} from "lucide-react";
import { StateDot, ViewHeader } from "@/components/common";
import { formatTimestamp } from "@/lib/utils";
import type { Snapshot } from "@/types";

function Metric({
  label,
  value,
  icon: Icon,
}: {
  label: string;
  value: string | number;
  icon: React.ComponentType<{ className?: string }>;
}) {
  return (
    <div className="metric-card">
      <div className="metric-icon">
        <Icon className="size-[18px]" />
      </div>
      <div>
        <div className="metric-value">{value}</div>
        <div className="metric-label">{label}</div>
      </div>
    </div>
  );
}

export function OverviewView({ snapshot }: { snapshot: Snapshot }) {
  const enabledPolicies = snapshot.policies.filter(
    (item) => item.enabled && item.available,
  );
  const compliantPolicies = enabledPolicies.filter((item) => item.compliant).length;
  const enabledPackages = snapshot.packages.filter((item) => item.enabled).length;
  const enabledProcesses = snapshot.settings.processRules.filter(
    (item) => item.enabled,
  ).length;
  const enabledNetwork = snapshot.networkBlocks.filter((item) => item.enabled);
  const blockedNetwork = enabledNetwork.filter((item) => item.blocked).length;
  const enabledTasks = snapshot.scheduledTasks.filter((item) => item.enabled);
  const disabledTasks = enabledTasks.filter((item) => item.disabled).length;

  return (
    <div className="view-stack">
      <ViewHeader title="Overview" />

      <section className="hero-status">
        <div className="hero-shield">
          <ShieldCheck className="size-7" />
        </div>
        <div className="min-w-0 flex-1">
          <h2>Policy is active</h2>
          <div className="mt-1 text-sm text-zinc-500">
            Last enforced {formatTimestamp(snapshot.status.lastEnforcedAt)}
          </div>
        </div>
        <div className="status-live">
          <span /> Running
        </div>
      </section>

      <section className="metrics-grid">
        <Metric
          label="Settings enforced"
          value={`${compliantPolicies}/${enabledPolicies.length}`}
          icon={Check}
        />
        <Metric label="App rules" value={enabledPackages} icon={Boxes} />
        <Metric label="Process rules" value={enabledProcesses} icon={Ban} />
        <Metric
          label="Changes reapplied"
          value={snapshot.status.repairedTotal}
          icon={RotateCcw}
        />
      </section>

      <div className="overview-columns">
        <section className="panel-card">
          <div className="panel-title">Enforcement</div>
          <div className="summary-list">
            <div className="summary-row">
              <div><StateDot active={compliantPolicies === enabledPolicies.length} /> Windows policy</div>
              <span>{compliantPolicies}/{enabledPolicies.length}</span>
            </div>
            <div className="summary-row">
              <div><StateDot active /> App removal</div>
              <span>{enabledPackages} rules</span>
            </div>
            <div className="summary-row">
              <div><StateDot active={disabledTasks === enabledTasks.length} /> Scheduled tasks</div>
              <span>{disabledTasks}/{enabledTasks.length}</span>
            </div>
            <div className="summary-row">
              <div><StateDot active={blockedNetwork === enabledNetwork.length} /> Network blocks</div>
              <span>{blockedNetwork}/{enabledNetwork.length}</span>
            </div>
            <div className="summary-row">
              <div><StateDot active /> Process blocker</div>
              <span>{snapshot.status.killedTotal} stopped</span>
            </div>
          </div>
        </section>

        <section className="panel-card">
          <div className="panel-title">Recent activity</div>
          <div className="recent-list">
            {snapshot.activity.slice(0, 5).map((event) => (
              <div className="recent-row" key={event.id}>
                <div className={event.success ? "event-icon success" : "event-icon error"}>
                  {event.success ? <Check className="size-3.5" /> : <Network className="size-3.5" />}
                </div>
                <div className="min-w-0 flex-1">
                  <div className="truncate text-sm text-zinc-300">{event.message}</div>
                  <div className="mt-0.5 text-xs text-zinc-600">
                    {formatTimestamp(event.timestamp)}
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
