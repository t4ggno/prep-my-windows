import { useMemo, useState } from "react";
import {
  BulkToggle,
  EmptyState,
  FilterPills,
  RuleSwitch,
  SearchField,
  SegmentedControl,
  StateDot,
  ViewHeader,
} from "@/components/common";
import { Select } from "@/components/ui/input";
import { cn, countBy, matchesQuery } from "@/lib/utils";
import type { PolicyItem, ScheduledTaskItem, TaskStatus } from "@/types";

type StatusFilter = "all" | "pending" | "off";

const ALL = "All";
const TASKS = "Scheduled tasks";

const taskStatusLabels: Record<TaskStatus, string> = {
  enabled: "Enabled",
  disabled: "Disabled",
  missing: "Not present",
  unknown: "Unknown",
};

function policyMatchesStatus(item: PolicyItem, filter: StatusFilter) {
  if (filter === "pending") return item.enabled && item.available && !item.compliant;
  if (filter === "off") return !item.enabled;
  return true;
}

function taskMatchesStatus(task: ScheduledTaskItem, filter: StatusFilter) {
  if (filter === "pending") return task.enabled && task.status === "enabled";
  if (filter === "off") return !task.enabled;
  return true;
}

export function PolicyView({
  policies,
  tasks,
  pending,
  onTogglePolicies,
  onToggleTasks,
  onChoosePolicyOption,
}: {
  policies: PolicyItem[];
  tasks: ScheduledTaskItem[];
  pending: ReadonlySet<string>;
  onTogglePolicies: (ids: string[], enabled: boolean) => void;
  onToggleTasks: (ids: string[], enabled: boolean) => void;
  onChoosePolicyOption: (id: string, choice: string) => void;
}) {
  const [query, setQuery] = useState("");
  const [category, setCategory] = useState(ALL);
  const [status, setStatus] = useState<StatusFilter>("all");

  const searchedPolicies = policies.filter((item) =>
    matchesQuery(query, item.name, item.category, item.current, item.wanted),
  );
  const searchedTasks = tasks.filter((task) => matchesQuery(query, task.name, task.path));
  const statusPolicies = searchedPolicies.filter((item) => policyMatchesStatus(item, status));
  const statusTasks = searchedTasks.filter((task) => taskMatchesStatus(task, status));

  const categoryCounts = countBy(statusPolicies, (item) => item.category);
  const categoryNames = useMemo(
    () => Array.from(new Set(policies.map((item) => item.category))),
    [policies],
  );
  const categories = [
    { id: ALL, label: ALL, count: statusPolicies.length + statusTasks.length },
    ...categoryNames.map((name) => ({ id: name, label: name, count: categoryCounts.get(name) ?? 0 })),
    { id: TASKS, label: TASKS, count: statusTasks.length },
  ];

  const visiblePolicies =
    category === TASKS
      ? []
      : statusPolicies.filter((item) => category === ALL || item.category === category);
  const visibleTasks = category === ALL || category === TASKS ? statusTasks : [];
  const bulkPolicies = visiblePolicies.filter((item) => item.available);

  function toggleVisible(enabled: boolean) {
    onTogglePolicies(
      bulkPolicies.filter((item) => item.enabled !== enabled).map((item) => item.id),
      enabled,
    );
    onToggleTasks(
      visibleTasks.filter((task) => task.enabled !== enabled).map((task) => task.id),
      enabled,
    );
  }
  const statusOptions = [
    { id: "all" as const, label: "All", count: searchedPolicies.length + searchedTasks.length },
    {
      id: "pending" as const,
      label: "Not applied",
      count:
        searchedPolicies.filter((item) => policyMatchesStatus(item, "pending")).length +
        searchedTasks.filter((task) => taskMatchesStatus(task, "pending")).length,
    },
    {
      id: "off" as const,
      label: "Off",
      count:
        searchedPolicies.filter((item) => !item.enabled).length +
        searchedTasks.filter((task) => !task.enabled).length,
    },
  ];

  return (
    <div className="view-stack">
      <ViewHeader
        title="Windows policy"
        actions={
          <>
            <SegmentedControl options={statusOptions} value={status} onChange={setStatus} />
            <SearchField value={query} onChange={setQuery} />
          </>
        }
      />

      <div className="flex items-start justify-between gap-3">
        <FilterPills options={categories} value={category} onChange={setCategory} />
        <BulkToggle
          items={[...bulkPolicies, ...visibleTasks]}
          pending={pending.has("policies:bulk") || pending.has("scheduledTasks:bulk")}
          onChange={toggleVisible}
        />
      </div>

      {visiblePolicies.length > 0 && (
        <section className="data-panel">
          <div className="data-header policy-grid">
            <span>Setting</span>
            <span>Scope</span>
            <span>Current</span>
            <span>Wanted</span>
            <span />
          </div>
          {visiblePolicies.map((item) => (
            <div className={cn("data-row policy-grid", !item.available && "unavailable")} key={item.id}>
              <div className="min-w-0">
                <div className="row-primary">
                  <StateDot active={item.available && (!item.enabled || item.compliant)} />
                  <span className="truncate">{item.name}</span>
                </div>
                <div className="row-secondary truncate pl-3.5">
                  {item.available ? item.category : (item.unavailableReason ?? "Unavailable")}
                </div>
              </div>
              <span className="row-secondary">{item.scope}</span>
              <code className={cn("truncate", item.enabled && item.available && !item.compliant && "text-amber-300/80")}>
                {item.available ? item.current : "Unavailable"}
              </code>
              {item.available && item.choices.length > 1 ? (
                <Select
                  className="h-7 rounded-md px-2 text-xs"
                  value={item.choice ?? ""}
                  disabled={pending.has(`choice:${item.id}`)}
                  aria-label={`${item.name} value`}
                  onChange={(event) => onChoosePolicyOption(item.id, event.target.value)}
                >
                  {item.choices.map((option) => (
                    <option key={option.id} value={option.id}>
                      {option.label}
                    </option>
                  ))}
                </Select>
              ) : (
                <code className="truncate">{item.available ? item.wanted : "—"}</code>
              )}
              <RuleSwitch
                checked={item.enabled}
                pending={pending.has(`policies:${item.id}`)}
                disabled={!item.available}
                onCheckedChange={(enabled) => onTogglePolicies([item.id], enabled)}
                label={`Enforce ${item.name}`}
              />
            </div>
          ))}
        </section>
      )}

      {visibleTasks.length > 0 && (
        <section className="data-panel">
          <div className="section-label">Scheduled tasks</div>
          {visibleTasks.map((task) => (
            <div className="data-row task-grid" key={task.id}>
              <div className="min-w-0">
                <div className="row-primary">
                  <StateDot active={!task.enabled || task.status === "disabled" || task.status === "missing"} />
                  <span className="truncate">{task.name}</span>
                </div>
                <div className="row-secondary truncate pl-3.5">{task.path}</div>
              </div>
              <span className={cn("state-text", task.status !== "enabled" && task.status !== "unknown" && "active")}>
                {taskStatusLabels[task.status]}
              </span>
              <RuleSwitch
                checked={task.enabled}
                pending={pending.has(`scheduledTasks:${task.id}`)}
                onCheckedChange={(enabled) => onToggleTasks([task.id], enabled)}
                label={`Disable ${task.name}`}
              />
            </div>
          ))}
        </section>
      )}

      {visiblePolicies.length === 0 && visibleTasks.length === 0 && (
        <section className="data-panel">
          <EmptyState>No matching settings</EmptyState>
        </section>
      )}
    </div>
  );
}
