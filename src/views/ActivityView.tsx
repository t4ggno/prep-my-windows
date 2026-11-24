import { useState } from "react";
import { Check, X } from "lucide-react";
import { EmptyState, SearchField, SegmentedControl, ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { formatTimestamp, matchesQuery } from "@/lib/utils";
import type { ActivityEvent } from "@/types";

type ActivityFilter = "all" | "errors";

export function ActivityView({
  activity,
  onClear,
}: {
  activity: ActivityEvent[];
  onClear: () => void;
}) {
  const [filter, setFilter] = useState<ActivityFilter>("all");
  const [query, setQuery] = useState("");
  const errorCount = activity.filter((event) => !event.success).length;
  const visible = activity.filter(
    (event) =>
      (filter === "all" || !event.success) && matchesQuery(query, event.message, event.kind),
  );

  return (
    <div className="view-stack">
      <ViewHeader
        title="Activity"
        actions={
          <>
            <SearchField value={query} onChange={setQuery} />
            <Button variant="secondary" size="sm" onClick={onClear} disabled={!activity.length}>
              Clear
            </Button>
          </>
        }
      />
      <SegmentedControl
        value={filter}
        onChange={setFilter}
        options={[
          { id: "all", label: "All", count: activity.length },
          { id: "errors", label: "Errors", count: errorCount },
        ]}
      />
      <section className="data-panel">
        {visible.map((event) => (
          <div className="activity-row" key={event.id}>
            <div className={event.success ? "event-icon success" : "event-icon error"}>
              {event.success ? <Check className="size-3.5" /> : <X className="size-3.5" />}
            </div>
            <div className="min-w-0 flex-1">
              <div className="whitespace-pre-line break-words text-sm text-zinc-200">
                {event.message}
              </div>
              <div className="mt-1 text-xs text-zinc-600">{event.kind}</div>
            </div>
            <time className="row-secondary" dateTime={event.timestamp}>
              {formatTimestamp(event.timestamp)}
            </time>
          </div>
        ))}
        {visible.length === 0 && (
          <EmptyState>{activity.length ? "No matching activity" : "No activity yet"}</EmptyState>
        )}
      </section>
    </div>
  );
}
