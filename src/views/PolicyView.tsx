import { useMemo, useState } from "react";
import { SearchField, StateDot, ViewHeader } from "@/components/common";
import { Switch } from "@/components/ui/switch";
import { cn } from "@/lib/utils";
import type { PolicyItem, ScheduledTaskItem } from "@/types";

export function PolicyView({
  policies,
  tasks,
  onTogglePolicy,
  onToggleTask,
}: {
  policies: PolicyItem[];
  tasks: ScheduledTaskItem[];
  onTogglePolicy: (id: string, enabled: boolean) => void;
  onToggleTask: (id: string, enabled: boolean) => void;
}) {
  const [query, setQuery] = useState("");
  const [category, setCategory] = useState("All");
  const categories = useMemo(
    () => ["All", ...Array.from(new Set(policies.map((item) => item.category)))],
    [policies],
  );
  const filtered = policies.filter((item) => {
    const matchesCategory = category === "All" || item.category === category;
    const normalized = query.trim().toLowerCase();
    return (
      matchesCategory &&
      (!normalized ||
        item.name.toLowerCase().includes(normalized) ||
        item.category.toLowerCase().includes(normalized))
    );
  });

  return (
    <div className="view-stack">
      <ViewHeader
        title="Windows policy"
        actions={<SearchField value={query} onChange={setQuery} />}
      />

      <div className="filter-pills">
        {categories.map((item) => (
          <button
            key={item}
            onClick={() => setCategory(item)}
            className={cn(category === item && "active")}
          >
            {item}
          </button>
        ))}
      </div>

      <section className="data-panel">
        <div className="data-header policy-grid">
          <span>Setting</span>
          <span>Scope</span>
          <span>Current</span>
          <span>Wanted</span>
          <span />
        </div>
        {filtered.map((item) => (
          <div className="data-row policy-grid" key={item.id}>
            <div className="min-w-0">
              <div className="row-primary">
                <StateDot
                  active={item.available && (!item.enabled || item.compliant)}
                />
                <span className="truncate">{item.name}</span>
              </div>
              <div className="row-secondary pl-3.5">
                {item.available
                  ? item.category
                  : item.unavailableReason ?? "Unavailable"}
              </div>
            </div>
            <span className="row-secondary">{item.scope}</span>
            <code>{item.available ? item.current : "Unavailable"}</code>
            <code>{item.available ? item.wanted : "—"}</code>
            <Switch
              checked={item.enabled}
              disabled={!item.available}
              onCheckedChange={(enabled) => onTogglePolicy(item.id, enabled)}
              aria-label={`Enforce ${item.name}`}
            />
          </div>
        ))}
      </section>

      {(category === "All" || category === "Diagnostics") && !query && (
        <section className="data-panel">
          <div className="section-label">Scheduled tasks</div>
          {tasks.map((task) => (
            <div className="data-row task-grid" key={task.id}>
              <div className="min-w-0">
                <div className="row-primary">
                  <StateDot active={!task.enabled || task.disabled} />
                  <span className="truncate">{task.name}</span>
                </div>
                <div className="row-secondary truncate pl-3.5">{task.path}</div>
              </div>
              <span className={cn("state-text", task.disabled && "active")}>
                {task.disabled ? "Disabled" : "Enabled"}
              </span>
              <Switch
                checked={task.enabled}
                onCheckedChange={(enabled) => onToggleTask(task.id, enabled)}
                aria-label={`Enforce ${task.name}`}
              />
            </div>
          ))}
        </section>
      )}
    </div>
  );
}
