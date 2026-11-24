import { useState } from "react";
import { LoaderCircle, RefreshCw, ShieldMinus, Trash2 } from "lucide-react";
import {
  EmptyState,
  FilterPills,
  LoadingState,
  SearchField,
  SegmentedControl,
  ViewHeader,
} from "@/components/common";
import { Button } from "@/components/ui/button";
import { cn, countBy, matchesQuery } from "@/lib/utils";
import type { AutostartEntry, AutostartKind, AutostartRule } from "@/types";

type KindFilter = AutostartKind | "all";

const kindLabels: Record<AutostartKind, string> = {
  registry: "Registry",
  startupFolder: "Startup folder",
  scheduledTask: "Scheduled task",
  service: "Service",
};

const kinds = Object.keys(kindLabels) as AutostartKind[];

export function AutostartView({
  entries,
  blocked,
  loading,
  pending,
  onRefresh,
  onBlock,
  onRemoveRule,
}: {
  entries: AutostartEntry[];
  blocked: AutostartRule[];
  loading: boolean;
  pending: ReadonlySet<string>;
  onRefresh: () => void;
  onBlock: (entry: AutostartEntry) => void;
  onRemoveRule: (id: string) => void;
}) {
  const [tab, setTab] = useState<"current" | "blocked">("current");
  const [kind, setKind] = useState<KindFilter>("all");
  const [query, setQuery] = useState("");
  const blockedIds = new Set(blocked.map((rule) => rule.id));
  const searched = entries.filter((entry) =>
    matchesQuery(query, entry.name, entry.command, entry.source),
  );
  const kindCounts = countBy(searched, (entry) => entry.kind);
  const visible = searched.filter((entry) => kind === "all" || entry.kind === kind);
  const visibleBlocked = blocked.filter((rule) => matchesQuery(query, rule.name, rule.location));

  return (
    <div className="view-stack">
      <ViewHeader
        title="Autostart"
        actions={
          <>
            <SearchField value={query} onChange={setQuery} />
            <Button
              variant="secondary"
              size="icon"
              onClick={onRefresh}
              disabled={loading}
              aria-label="Refresh"
            >
              <RefreshCw className={cn("size-4", loading && "animate-spin")} />
            </Button>
          </>
        }
      />
      <SegmentedControl
        value={tab}
        onChange={setTab}
        options={[
          { id: "current", label: "Current entries", count: entries.length },
          { id: "blocked", label: "Blocked", count: blocked.length },
        ]}
      />

      {tab === "current" ? (
        <>
          <FilterPills
            value={kind}
            onChange={setKind}
            options={[
              { id: "all", label: "All", count: searched.length },
              ...kinds.map((id) => ({ id, label: kindLabels[id], count: kindCounts.get(id) ?? 0 })),
            ]}
          />
          <section className="data-panel">
            <div className="data-header autostart-grid">
              <span>Entry</span>
              <span>Source</span>
              <span>State</span>
              <span />
            </div>
            {visible.map((entry) => {
              const isBlocked = blockedIds.has(entry.id);
              const blocking = pending.has(`autostart:${entry.id}`);
              return (
                <div className="data-row autostart-grid" key={entry.id}>
                  <div className="min-w-0">
                    <div className="row-primary truncate">{entry.name}</div>
                    <div className="row-secondary truncate" title={entry.command}>
                      {entry.command}
                    </div>
                  </div>
                  <span className="row-secondary truncate">{entry.source}</span>
                  <span className="state-text">{entry.state}</span>
                  <Button
                    variant="secondary"
                    size="sm"
                    disabled={isBlocked || blocking}
                    onClick={() => onBlock(entry)}
                  >
                    {blocking ? (
                      <LoaderCircle className="size-3.5 animate-spin" />
                    ) : (
                      <ShieldMinus className="size-3.5" />
                    )}
                    {isBlocked ? "Blocked" : "Block"}
                  </Button>
                </div>
              );
            })}
            {loading && entries.length === 0 && <LoadingState />}
            {!loading && visible.length === 0 && <EmptyState>No matching entries</EmptyState>}
          </section>
        </>
      ) : (
        <section className="data-panel">
          {visibleBlocked.map((rule) => (
            <div className="data-row blocked-grid" key={rule.id}>
              <div className="min-w-0">
                <div className="row-primary truncate">{rule.name}</div>
                <div className="row-secondary truncate" title={rule.location}>
                  {rule.location}
                </div>
              </div>
              <span className="row-secondary">{kindLabels[rule.kind]}</span>
              <Button
                variant="ghost"
                size="icon"
                disabled={pending.has(`autostart-rule:${rule.id}`)}
                onClick={() => onRemoveRule(rule.id)}
                aria-label={`Remove ${rule.name} rule`}
              >
                <Trash2 className="size-4" />
              </Button>
            </div>
          ))}
          {visibleBlocked.length === 0 && <EmptyState>No blocked entries</EmptyState>}
        </section>
      )}
    </div>
  );
}
