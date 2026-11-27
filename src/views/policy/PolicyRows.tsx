import { RuleSwitch, StateDot } from "@/components/common";
import { cn } from "@/lib/utils";
import type { PolicyItem, ScheduledTaskItem } from "@/types";
import { policyIsMismatched } from "@/views/policy/policyFilters";
import { taskStatusLabels } from "@/views/policy/SettingInfoDialog";

function RowTitle({
  name,
  changed,
  onOpen,
}: {
  name: string;
  changed: boolean;
  onOpen: () => void;
}) {
  return (
    <>
      <button className="row-title" onClick={onOpen}>
        {name}
      </button>
      {changed && <span className="changed-mark">Changed</span>}
    </>
  );
}

export function PolicyHeader() {
  return (
    <div className="data-header policy-grid">
      <span>Setting</span>
      <span>Scope</span>
      <span>Current</span>
      <span>Wanted</span>
      <span />
    </div>
  );
}

export function PolicyRow({
  item,
  note,
  changed,
  editor,
  pending,
  onOpen,
  onToggle,
}: {
  item: PolicyItem;
  note: string;
  changed: boolean;
  editor: React.ReactNode;
  pending: ReadonlySet<string>;
  onOpen: () => void;
  onToggle: (enabled: boolean) => void;
}) {
  return (
    <div className={cn("data-row policy-grid", !item.available && "unavailable")}>
      <div className="min-w-0">
        <div className="row-primary">
          <StateDot active={item.available && (!item.enabled || item.compliant)} />
          <RowTitle name={item.name} changed={changed} onOpen={onOpen} />
        </div>
        {note && <div className="row-secondary truncate pl-3.5">{note}</div>}
      </div>
      <span className="row-secondary">{item.scope}</span>
      <code className={cn("truncate", item.enabled && policyIsMismatched(item) && "text-amber-300/80")}>
        {item.available ? item.current : "Unavailable"}
      </code>
      {editor}
      <RuleSwitch
        checked={item.enabled}
        pending={pending.has(`policies:${item.id}`)}
        disabled={!item.available}
        onCheckedChange={onToggle}
        label={`Enforce ${item.name}`}
      />
    </div>
  );
}

export function TaskRow({
  task,
  changed,
  pending,
  onOpen,
  onToggle,
}: {
  task: ScheduledTaskItem;
  changed: boolean;
  pending: ReadonlySet<string>;
  onOpen: () => void;
  onToggle: (enabled: boolean) => void;
}) {
  return (
    <div className="data-row task-grid">
      <div className="min-w-0">
        <div className="row-primary">
          <StateDot
            active={!task.enabled || task.status === "disabled" || task.status === "missing"}
          />
          <RowTitle name={task.name} changed={changed} onOpen={onOpen} />
        </div>
        <div className="row-secondary truncate pl-3.5">{task.path}</div>
      </div>
      <span
        className={cn(
          "state-text",
          task.status !== "enabled" && task.status !== "unknown" && "active",
        )}
      >
        {taskStatusLabels[task.status]}
      </span>
      <RuleSwitch
        checked={task.enabled}
        pending={pending.has(`scheduledTasks:${task.id}`)}
        onCheckedChange={onToggle}
        label={`Disable ${task.name}`}
      />
    </div>
  );
}
