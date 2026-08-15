import { Check, X } from "lucide-react";
import { ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { formatTimestamp } from "@/lib/utils";
import type { ActivityEvent } from "@/types";

export function ActivityView({
  activity,
  onClear,
}: {
  activity: ActivityEvent[];
  onClear: () => void;
}) {
  return (
    <div className="view-stack">
      <ViewHeader
        title="Activity"
        actions={
          <Button variant="secondary" size="sm" onClick={onClear} disabled={!activity.length}>
            Clear
          </Button>
        }
      />
      <section className="data-panel">
        {activity.map((event) => (
          <div className="activity-row" key={event.id}>
            <div className={event.success ? "event-icon success" : "event-icon error"}>
              {event.success ? <Check className="size-3.5" /> : <X className="size-3.5" />}
            </div>
            <div className="min-w-0 flex-1">
              <div className="text-sm text-zinc-200">{event.message}</div>
              <div className="mt-1 text-xs text-zinc-600">{event.kind}</div>
            </div>
            <time className="row-secondary">{formatTimestamp(event.timestamp)}</time>
          </div>
        ))}
        {!activity.length && <div className="empty-state">No activity yet</div>}
      </section>
    </div>
  );
}
