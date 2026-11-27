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
import { cn, countBy, matchesQuery } from "@/lib/utils";
import type { ActiveHours, PolicyItem, ScheduledTaskItem } from "@/types";
import { PolicyValueEditor } from "@/views/policy/PolicyValueEditor";
import {
  policyIsChanged,
  policyIsMismatched,
  policyMatches,
  ruleDiffersFromRecommendation,
  type RuleFilter,
  type StateFilter,
  taskIsChanged,
  taskMatches,
  valueDiffersFromRecommendation,
} from "@/views/policy/policyFilters";
import {
  type InfoTarget,
  SettingInfoDialog,
  taskStatusLabels,
} from "@/views/policy/SettingInfoDialog";

const ALL = "All";
const TASKS = "Scheduled tasks";

const ruleOptions: Array<{ id: RuleFilter; label: string }> = [
  { id: "all", label: "All rules" },
  { id: "on", label: "Enforced" },
  { id: "off", label: "Not enforced" },
];

const stateOptions: Array<{ id: StateFilter; label: string }> = [
  { id: "all", label: "Any state" },
  { id: "mismatch", label: "Mismatch" },
  { id: "matches", label: "Matches" },
  { id: "changed", label: "Changed" },
  { id: "unavailable", label: "Unavailable" },
];

type InfoSelection = { kind: InfoTarget["kind"]; id: string };

function policyNote(item: PolicyItem) {
  if (!item.available) return item.unavailableReason ?? "Unavailable";
  if (valueDiffersFromRecommendation(item)) {
    return `${item.category} · Recommended: ${item.recommended}`;
  }
  if (ruleDiffersFromRecommendation(item)) {
    return `${item.category} · ${item.recommendedEnabled ? "Recommended to enforce" : "Optional rule"}`;
  }
  return item.category;
}

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

function resolveInfoTarget(
  selection: InfoSelection | null,
  policies: PolicyItem[],
  tasks: ScheduledTaskItem[],
): InfoTarget | null {
  if (!selection) return null;
  if (selection.kind === "policy") {
    const item = policies.find((policy) => policy.id === selection.id);
    return item ? { kind: "policy", item } : null;
  }
  const item = tasks.find((task) => task.id === selection.id);
  return item ? { kind: "task", item } : null;
}

export function PolicyView({
  policies,
  tasks,
  activeHours,
  pending,
  onTogglePolicies,
  onToggleTasks,
  onChoosePolicyOption,
  onChangeActiveHours,
}: {
  policies: PolicyItem[];
  tasks: ScheduledTaskItem[];
  activeHours: ActiveHours;
  pending: ReadonlySet<string>;
  onTogglePolicies: (ids: string[], enabled: boolean) => void;
  onToggleTasks: (ids: string[], enabled: boolean) => void;
  onChoosePolicyOption: (id: string, choice: string) => void;
  onChangeActiveHours: (hours: ActiveHours) => void;
}) {
  const [query, setQuery] = useState("");
  const [category, setCategory] = useState(ALL);
  const [rule, setRule] = useState<RuleFilter>("all");
  const [state, setState] = useState<StateFilter>("all");
  const [info, setInfo] = useState<InfoSelection | null>(null);

  const searchedPolicies = policies.filter((item) =>
    matchesQuery(query, item.name, item.category, item.current, item.wanted, item.recommended),
  );
  const searchedTasks = tasks.filter((task) => matchesQuery(query, task.name, task.path));
  const countMatches = (ruleFilter: RuleFilter, stateFilter: StateFilter) =>
    searchedPolicies.filter((item) => policyMatches(item, ruleFilter, stateFilter)).length +
    searchedTasks.filter((task) => taskMatches(task, ruleFilter, stateFilter)).length;

  const filteredPolicies = searchedPolicies.filter((item) => policyMatches(item, rule, state));
  const filteredTasks = searchedTasks.filter((task) => taskMatches(task, rule, state));

  const categoryCounts = countBy(filteredPolicies, (item) => item.category);
  const categoryNames = useMemo(
    () => Array.from(new Set(policies.map((item) => item.category))),
    [policies],
  );
  const categories = [
    { id: ALL, label: ALL, count: filteredPolicies.length + filteredTasks.length },
    ...categoryNames.map((name) => ({ id: name, label: name, count: categoryCounts.get(name) ?? 0 })),
    { id: TASKS, label: TASKS, count: filteredTasks.length },
  ];

  const visiblePolicies =
    category === TASKS
      ? []
      : filteredPolicies.filter((item) => category === ALL || item.category === category);
  const visibleTasks = category === ALL || category === TASKS ? filteredTasks : [];
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

  return (
    <div className="view-stack">
      <ViewHeader
        title="Windows policy"
        actions={<SearchField value={query} onChange={setQuery} />}
      />

      <div className="policy-toolbar">
        <SegmentedControl
          options={ruleOptions.map((option) => ({ ...option, count: countMatches(option.id, state) }))}
          value={rule}
          onChange={setRule}
        />
        <SegmentedControl
          options={stateOptions.map((option) => ({ ...option, count: countMatches(rule, option.id) }))}
          value={state}
          onChange={setState}
        />
        <BulkToggle
          items={[...bulkPolicies, ...visibleTasks]}
          pending={pending.has("policies:bulk") || pending.has("scheduledTasks:bulk")}
          onChange={toggleVisible}
        />
      </div>

      <FilterPills options={categories} value={category} onChange={setCategory} />

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
                  <RowTitle
                    name={item.name}
                    changed={policyIsChanged(item)}
                    onOpen={() => setInfo({ kind: "policy", id: item.id })}
                  />
                </div>
                <div className="row-secondary truncate pl-3.5">{policyNote(item)}</div>
              </div>
              <span className="row-secondary">{item.scope}</span>
              <code
                className={cn(
                  "truncate",
                  item.enabled && policyIsMismatched(item) && "text-amber-300/80",
                )}
              >
                {item.available ? item.current : "Unavailable"}
              </code>
              <PolicyValueEditor
                item={item}
                activeHours={activeHours}
                choicePending={pending.has(`choice:${item.id}`)}
                activeHoursPending={pending.has("settings")}
                onChoose={(choice) => onChoosePolicyOption(item.id, choice)}
                onChangeActiveHours={onChangeActiveHours}
              />
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
                  <StateDot
                    active={!task.enabled || task.status === "disabled" || task.status === "missing"}
                  />
                  <RowTitle
                    name={task.name}
                    changed={taskIsChanged(task)}
                    onOpen={() => setInfo({ kind: "task", id: task.id })}
                  />
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

      <SettingInfoDialog
        target={resolveInfoTarget(info, policies, tasks)}
        onClose={() => setInfo(null)}
      />
    </div>
  );
}
