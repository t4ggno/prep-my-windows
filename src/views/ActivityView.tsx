import { useState } from "react";
import { Check, RotateCcw, X } from "lucide-react";
import { EmptyState, SearchField, SegmentedControl, ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { formatTimestamp, matchesQuery } from "@/lib/utils";
import type { ActivityEvent, ChangeSection, ReappliedChange } from "@/types";

export type ActivityFilter = "all" | "reapplied" | "errors";

const sectionLabels: Record<ChangeSection, string> = {
  setting: "Windows policy",
  app: "App",
  network: "Network",
  scheduledTask: "Scheduled task",
  autostart: "Autostart",
};

function ActivityRow({ event }: { event: ActivityEvent }) {
  return (
    <div className="activity-row">
      <div className={event.success ? "event-icon success" : "event-icon error"}>
        {event.success ? <Check className="size-3.5" /> : <X className="size-3.5" />}
      </div>
      <div className="min-w-0 flex-1">
        <div className="whitespace-pre-line break-words text-sm text-zinc-200">
          {event.message}
        </div>
        {event.details.length > 0 && (
          <ul className="mt-1 space-y-0.5 text-xs text-zinc-400">
            {event.details.map((detail) => (
              <li className="break-words" key={detail}>
                {detail}
              </li>
            ))}
          </ul>
        )}
        <div className="mt-1 text-xs text-zinc-600">{event.kind}</div>
      </div>
      <time className="row-secondary" dateTime={event.timestamp}>
        {formatTimestamp(event.timestamp)}
      </time>
    </div>
  );
}

function ReappliedRow({ change }: { change: ReappliedChange }) {
  return (
    <div className="activity-row">
      <div className="event-icon reapplied">
        <RotateCcw className="size-3.5" />
      </div>
      <div className="min-w-0 flex-1">
        <div className="break-words text-sm text-zinc-200">{change.name}</div>
        <div className="mt-1 break-words text-xs text-zinc-400">
          {change.lastFound} → {change.wanted}
        </div>
        <div className="mt-1 text-xs text-zinc-600">{sectionLabels[change.section]}</div>
      </div>
      <div className="text-right">
        <div className="text-sm font-medium tabular-nums text-zinc-200">{change.reverts}×</div>
        <time className="row-secondary block" dateTime={change.lastRevertedAt}>
          {formatTimestamp(change.lastRevertedAt)}
        </time>
      </div>
    </div>
  );
}

export function ActivityView({
  activity,
  reapplied,
  filter,
  onFilterChange,
  onClear,
}: {
  activity: ActivityEvent[];
  reapplied: ReappliedChange[];
  filter: ActivityFilter;
  onFilterChange: (filter: ActivityFilter) => void;
  onClear: () => void;
}) {
  const [query, setQuery] = useState("");
  const errorCount = activity.filter((event) => !event.success).length;
  const visibleEvents = activity.filter(
    (event) =>
      (filter === "all" || !event.success) &&
      matchesQuery(query, event.message, event.kind, ...event.details),
  );
  const visibleChanges = reapplied.filter((change) =>
    matchesQuery(
      query,
      change.name,
      sectionLabels[change.section],
      change.lastFound,
      change.wanted,
    ),
  );

  return (
    <div className="view-stack">
      <ViewHeader
        title="Activity"
        actions={
          <>
            <SearchField value={query} onChange={setQuery} />
            {filter !== "reapplied" && (
              <Button variant="secondary" size="sm" onClick={onClear} disabled={!activity.length}>
                Clear
              </Button>
            )}
          </>
        }
      />
      <SegmentedControl
        value={filter}
        onChange={onFilterChange}
        options={[
          { id: "all", label: "All", count: activity.length },
          { id: "reapplied", label: "Reapplied", count: reapplied.length },
          { id: "errors", label: "Errors", count: errorCount },
        ]}
      />
      <section className="data-panel">
        {filter === "reapplied" ? (
          <>
            {visibleChanges.map((change) => (
              <ReappliedRow change={change} key={change.key} />
            ))}
            {visibleChanges.length === 0 && (
              <EmptyState>
                {reapplied.length ? "No matching changes" : "Nothing changed back yet"}
              </EmptyState>
            )}
          </>
        ) : (
          <>
            {visibleEvents.map((event) => (
              <ActivityRow event={event} key={event.id} />
            ))}
            {visibleEvents.length === 0 && (
              <EmptyState>{activity.length ? "No matching activity" : "No activity yet"}</EmptyState>
            )}
          </>
        )}
      </section>
    </div>
  );
}
