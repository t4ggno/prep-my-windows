import { useState } from "react";
import {
  BulkToggle,
  EmptyState,
  RuleSwitch,
  SearchField,
  StateDot,
  ViewHeader,
} from "@/components/common";
import { matchesQuery } from "@/lib/utils";
import type { NetworkItem } from "@/types";

export function NetworkView({
  items,
  pending,
  onToggle,
}: {
  items: NetworkItem[];
  pending: ReadonlySet<string>;
  onToggle: (ids: string[], enabled: boolean) => void;
}) {
  const [query, setQuery] = useState("");
  const filtered = items.filter((item) => matchesQuery(query, item.domain));

  return (
    <div className="view-stack">
      <ViewHeader
        title="Network"
        actions={
          <>
            <BulkToggle
              items={filtered}
              pending={pending.has("networkBlocks:bulk")}
              onChange={(enabled) =>
                onToggle(
                  filtered.filter((item) => item.enabled !== enabled).map((item) => item.id),
                  enabled,
                )
              }
            />
            <SearchField value={query} onChange={setQuery} />
          </>
        }
      />
      <section className="data-panel">
        <div className="data-header network-grid">
          <span>Endpoint</span>
          <span>State</span>
          <span />
        </div>
        {filtered.map((item) => (
          <div className="data-row network-grid" key={item.id}>
            <div className="row-primary min-w-0">
              <StateDot active={!item.enabled || item.blocked} />
              <code className="truncate">{item.domain}</code>
            </div>
            <span className={item.blocked ? "state-text active" : "state-text"}>
              {item.blocked ? "Blocked" : "Open"}
            </span>
            <RuleSwitch
              checked={item.enabled}
              pending={pending.has(`networkBlocks:${item.id}`)}
              onCheckedChange={(enabled) => onToggle([item.id], enabled)}
              label={`Block ${item.domain}`}
            />
          </div>
        ))}
        {filtered.length === 0 && <EmptyState>No matching endpoints</EmptyState>}
      </section>
    </div>
  );
}
