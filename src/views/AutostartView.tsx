import { useState } from "react";
import { RefreshCw, ShieldMinus, Trash2 } from "lucide-react";
import { EmptyState, SearchField, ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import type { AutostartEntry, AutostartRule } from "@/types";

export function AutostartView({
  entries,
  blocked,
  loading,
  onRefresh,
  onBlock,
  onRemoveRule,
}: {
  entries: AutostartEntry[];
  blocked: AutostartRule[];
  loading: boolean;
  onRefresh: () => void;
  onBlock: (entry: AutostartEntry) => void;
  onRemoveRule: (id: string) => void;
}) {
  const [tab, setTab] = useState<"current" | "blocked">("current");
  const [query, setQuery] = useState("");
  const blockedIds = new Set(blocked.map((rule) => rule.id));
  const filtered = entries.filter((entry) => {
    const value = query.trim().toLowerCase();
    return (
      !value ||
      entry.name.toLowerCase().includes(value) ||
      entry.command.toLowerCase().includes(value) ||
      entry.source.toLowerCase().includes(value)
    );
  });

  return (
    <div className="view-stack">
      <ViewHeader
        title="Autostart"
        actions={
          <>
            <SearchField value={query} onChange={setQuery} />
            <Button variant="secondary" size="icon" onClick={onRefresh} disabled={loading}>
              <RefreshCw className={cn("size-4", loading && "animate-spin")} />
            </Button>
          </>
        }
      />
      <div className="segmented-control">
        <button className={cn(tab === "current" && "active")} onClick={() => setTab("current")}>
          Current entries
        </button>
        <button className={cn(tab === "blocked" && "active")} onClick={() => setTab("blocked")}>
          Blocked
        </button>
      </div>

      {tab === "current" ? (
        <section className="data-panel">
          <div className="data-header autostart-grid">
            <span>Entry</span>
            <span>Source</span>
            <span>State</span>
            <span />
          </div>
          {filtered.map((entry) => (
            <div className="data-row autostart-grid" key={entry.id}>
              <div className="min-w-0">
                <div className="row-primary truncate">{entry.name}</div>
                <div className="row-secondary truncate">{entry.command}</div>
              </div>
              <span className="row-secondary truncate">{entry.source}</span>
              <span className="state-text">{entry.state}</span>
              <Button
                variant="secondary"
                size="sm"
                disabled={blockedIds.has(entry.id)}
                onClick={() => onBlock(entry)}
              >
                <ShieldMinus className="size-3.5" />
                {blockedIds.has(entry.id) ? "Blocked" : "Block"}
              </Button>
            </div>
          ))}
          {!loading && filtered.length === 0 && <EmptyState>No entries found</EmptyState>}
        </section>
      ) : (
        <section className="data-panel">
          {blocked.map((rule) => (
            <div className="data-row blocked-grid" key={rule.id}>
              <div className="min-w-0">
                <div className="row-primary truncate">{rule.name}</div>
                <div className="row-secondary truncate">{rule.location}</div>
              </div>
              <span className="row-secondary">{rule.kind}</span>
              <Button
                variant="ghost"
                size="icon"
                onClick={() => onRemoveRule(rule.id)}
                aria-label={`Remove ${rule.name} rule`}
              >
                <Trash2 className="size-4" />
              </Button>
            </div>
          ))}
          {!blocked.length && <EmptyState>No blocked entries</EmptyState>}
        </section>
      )}
    </div>
  );
}
