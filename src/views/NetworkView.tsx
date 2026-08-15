import { useState } from "react";
import { SearchField, StateDot, ViewHeader } from "@/components/common";
import { Switch } from "@/components/ui/switch";
import type { NetworkItem } from "@/types";

export function NetworkView({
  items,
  onToggle,
}: {
  items: NetworkItem[];
  onToggle: (id: string, enabled: boolean) => void;
}) {
  const [query, setQuery] = useState("");
  const filtered = items.filter((item) =>
    item.domain.toLowerCase().includes(query.trim().toLowerCase()),
  );

  return (
    <div className="view-stack">
      <ViewHeader
        title="Network"
        actions={<SearchField value={query} onChange={setQuery} />}
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
            <Switch
              checked={item.enabled}
              onCheckedChange={(enabled) => onToggle(item.id, enabled)}
              aria-label={`Block ${item.domain}`}
            />
          </div>
        ))}
      </section>
    </div>
  );
}
