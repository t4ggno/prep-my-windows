import { useMemo, useState } from "react";
import {
  BulkToggle,
  EmptyState,
  FilterPills,
  SearchField,
  SegmentedControl,
  ViewHeader,
} from "@/components/common";
import { countBy, matchesQuery } from "@/lib/utils";
import type { ActiveHours, PolicyItem, ScheduledTaskItem } from "@/types";
import { PolicyHeader, PolicyRow, TaskRow } from "@/views/policy/PolicyRows";
import { PolicyValueEditor } from "@/views/policy/PolicyValueEditor";
import {
  policyIsChanged,
  policyMatches,
  ruleDiffersFromRecommendation,
  type RuleFilter,
  type StateFilter,
  taskIsChanged,
  taskMatches,
  valueDiffersFromRecommendation,
} from "@/views/policy/policyFilters";
import {
  type InfoSelection,
  resolveInfoTarget,
  SettingInfoDialog,
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
          <PolicyHeader />
          {visiblePolicies.map((item) => (
            <PolicyRow
              key={item.id}
              item={item}
              note={policyNote(item)}
              changed={policyIsChanged(item)}
              editor={
                <PolicyValueEditor
                  item={item}
                  activeHours={activeHours}
                  choicePending={pending.has(`choice:${item.id}`)}
                  activeHoursPending={pending.has("settings")}
                  onChoose={(choice) => onChoosePolicyOption(item.id, choice)}
                  onChangeActiveHours={onChangeActiveHours}
                />
              }
              pending={pending}
              onOpen={() => setInfo({ kind: "policy", id: item.id })}
              onToggle={(enabled) => onTogglePolicies([item.id], enabled)}
            />
          ))}
        </section>
      )}

      {visibleTasks.length > 0 && (
        <section className="data-panel">
          <div className="section-label">Scheduled tasks</div>
          {visibleTasks.map((task) => (
            <TaskRow
              key={task.id}
              task={task}
              changed={taskIsChanged(task)}
              pending={pending}
              onOpen={() => setInfo({ kind: "task", id: task.id })}
              onToggle={(enabled) => onToggleTasks([task.id], enabled)}
            />
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
