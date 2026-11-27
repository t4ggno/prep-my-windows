import { useState } from "react";
import {
  BulkToggle,
  EmptyState,
  RuleSwitch,
  SearchField,
  SegmentedControl,
  ViewHeader,
} from "@/components/common";
import { matchesQuery } from "@/lib/utils";
import type { PolicyItem, ScheduledTaskItem, SoftwareItem } from "@/types";
import { PolicyHeader, PolicyRow, TaskRow } from "@/views/policy/PolicyRows";
import { valueDiffersFromRecommendation } from "@/views/policy/policyFilters";
import { PolicyChoiceEditor } from "@/views/policy/PolicyValueEditor";
import {
  type InfoSelection,
  resolveInfoTarget,
  SettingInfoDialog,
} from "@/views/policy/SettingInfoDialog";

type InstallFilter = "installed" | "all";

interface Bundle {
  software: SoftwareItem;
  policies: PolicyItem[];
  tasks: ScheduledTaskItem[];
}

function fixNote(item: PolicyItem) {
  if (!item.available) return item.unavailableReason ?? "Unavailable";
  return valueDiffersFromRecommendation(item) ? `Recommended: ${item.recommended}` : "";
}

function switchablePolicies(bundle: Bundle) {
  return bundle.policies.filter((item) => item.available);
}

function switchableTasks(bundle: Bundle) {
  return bundle.software.installed ? bundle.tasks : [];
}

function switchableItems(bundle: Bundle) {
  return [...switchablePolicies(bundle), ...switchableTasks(bundle)];
}

function bundleMatches(bundle: Bundle, query: string) {
  return (
    matchesQuery(query, bundle.software.name) ||
    bundle.policies.some((item) => matchesQuery(query, item.name)) ||
    bundle.tasks.some((task) => matchesQuery(query, task.name, task.path))
  );
}

export function SoftwareView({
  software,
  policies,
  tasks,
  pending,
  onTogglePolicies,
  onToggleTasks,
  onChoosePolicyOption,
}: {
  software: SoftwareItem[];
  policies: PolicyItem[];
  tasks: ScheduledTaskItem[];
  pending: ReadonlySet<string>;
  onTogglePolicies: (ids: string[], enabled: boolean) => void;
  onToggleTasks: (ids: string[], enabled: boolean) => void;
  onChoosePolicyOption: (id: string, choice: string) => void;
}) {
  const [query, setQuery] = useState("");
  const [filter, setFilter] = useState<InstallFilter>("installed");
  const [info, setInfo] = useState<InfoSelection | null>(null);

  const bundles = software
    .map((item) => ({
      software: item,
      policies: policies.filter((policy) => policy.software === item.id),
      tasks: tasks.filter((task) => task.software === item.id),
    }))
    .filter((bundle) => bundleMatches(bundle, query))
    .sort((left, right) => Number(right.software.installed) - Number(left.software.installed));
  const installedCount = bundles.filter((bundle) => bundle.software.installed).length;
  const visibleBundles =
    filter === "installed" ? bundles.filter((bundle) => bundle.software.installed) : bundles;
  const bulkPending = pending.has("policies:bulk") || pending.has("scheduledTasks:bulk");

  function toggleBundles(selected: Bundle[], enabled: boolean) {
    onTogglePolicies(
      selected
        .flatMap(switchablePolicies)
        .filter((item) => item.enabled !== enabled)
        .map((item) => item.id),
      enabled,
    );
    onToggleTasks(
      selected
        .flatMap(switchableTasks)
        .filter((task) => task.enabled !== enabled)
        .map((task) => task.id),
      enabled,
    );
  }

  return (
    <div className="view-stack">
      <ViewHeader title="Software" actions={<SearchField value={query} onChange={setQuery} />} />

      <div className="policy-toolbar">
        <SegmentedControl
          options={[
            { id: "installed", label: "Installed", count: installedCount },
            { id: "all", label: "All", count: bundles.length },
          ]}
          value={filter}
          onChange={setFilter}
        />
        <BulkToggle
          items={visibleBundles.flatMap(switchableItems)}
          pending={bulkPending}
          onChange={(enabled) => toggleBundles(visibleBundles, enabled)}
        />
      </div>

      {visibleBundles.map((bundle) => {
        const items = switchableItems(bundle);
        const enabledCount = items.filter((item) => item.enabled).length;
        const bundlePending =
          bulkPending ||
          bundle.policies.some((item) => pending.has(`policies:${item.id}`)) ||
          bundle.tasks.some((task) => pending.has(`scheduledTasks:${task.id}`));
        return (
          <section className="data-panel" key={bundle.software.id}>
            <div className="data-row bundle-grid bundle-row">
              <div className="min-w-0">
                <div className="bundle-title">{bundle.software.name}</div>
                <div className="row-secondary">
                  {bundle.software.installed
                    ? `${enabledCount} of ${items.length} on`
                    : "Not installed on this PC"}
                </div>
              </div>
              <RuleSwitch
                checked={items.length > 0 && enabledCount === items.length}
                pending={bundlePending}
                disabled={items.length === 0}
                onCheckedChange={(enabled) => toggleBundles([bundle], enabled)}
                label={`Apply all ${bundle.software.name} fixes`}
              />
            </div>
            {bundle.policies.length > 0 && <PolicyHeader />}
            {bundle.policies.map((item) => (
              <PolicyRow
                key={item.id}
                item={item}
                note={fixNote(item)}
                changed={valueDiffersFromRecommendation(item)}
                editor={
                  <PolicyChoiceEditor
                    item={item}
                    pending={pending.has(`choice:${item.id}`)}
                    onChoose={(choice) => onChoosePolicyOption(item.id, choice)}
                  />
                }
                pending={pending}
                onOpen={() => setInfo({ kind: "policy", id: item.id })}
                onToggle={(enabled) => onTogglePolicies([item.id], enabled)}
              />
            ))}
            {bundle.tasks.map((task) => (
              <TaskRow
                key={task.id}
                task={task}
                changed={false}
                pending={pending}
                onOpen={() => setInfo({ kind: "task", id: task.id })}
                onToggle={(enabled) => onToggleTasks([task.id], enabled)}
              />
            ))}
          </section>
        );
      })}

      {visibleBundles.length === 0 && (
        <section className="data-panel">
          <EmptyState>No matching software</EmptyState>
        </section>
      )}

      <SettingInfoDialog
        target={resolveInfoTarget(info, policies, tasks)}
        onClose={() => setInfo(null)}
      />
    </div>
  );
}
