import { useMemo, useState } from "react";
import { Plus, Trash2 } from "lucide-react";
import { SearchField, StateDot, ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { cn } from "@/lib/utils";
import type { InstalledPackage, PackageItem } from "@/types";

export function AppsView({
  rules,
  installed,
  customPackages,
  onToggle,
  onAddCustom,
  onRemoveCustom,
}: {
  rules: PackageItem[];
  installed: InstalledPackage[];
  customPackages: string[];
  onToggle: (id: string, enabled: boolean) => void;
  onAddCustom: (name: string) => void;
  onRemoveCustom: (name: string) => void;
}) {
  const [tab, setTab] = useState<"rules" | "installed">("rules");
  const [query, setQuery] = useState("");
  const managedNames = useMemo(
    () =>
      new Set([
        ...rules.map((item) => item.packageName.toLowerCase()),
        ...customPackages.map((item) => item.toLowerCase()),
      ]),
    [customPackages, rules],
  );
  const normalized = query.trim().toLowerCase();
  const filteredRules = rules.filter(
    (item) =>
      !normalized ||
      item.name.toLowerCase().includes(normalized) ||
      item.packageName.toLowerCase().includes(normalized),
  );
  const filteredInstalled = installed.filter(
    (item) =>
      !normalized ||
      item.name.toLowerCase().includes(normalized) ||
      item.publisher.toLowerCase().includes(normalized),
  );

  return (
    <div className="view-stack">
      <ViewHeader
        title="Apps"
        actions={<SearchField value={query} onChange={setQuery} />}
      />
      <div className="segmented-control">
        <button className={cn(tab === "rules" && "active")} onClick={() => setTab("rules")}>
          Removal rules
        </button>
        <button className={cn(tab === "installed" && "active")} onClick={() => setTab("installed")}>
          Installed
        </button>
      </div>

      {tab === "rules" ? (
        <>
          <section className="data-panel">
            <div className="data-header app-grid">
              <span>App</span>
              <span>Package</span>
              <span>State</span>
              <span />
            </div>
            {filteredRules.map((item) => (
              <div className="data-row app-grid" key={item.id}>
                <div className="row-primary min-w-0">
                  <StateDot active={!item.enabled || !item.installed} />
                  <span className="truncate">{item.name}</span>
                </div>
                <code className="truncate">{item.packageName}</code>
                <span className={!item.installed ? "state-text active" : "state-text"}>
                  {item.installed ? (item.provisioned ? "Provisioned" : "Installed") : "Absent"}
                </span>
                <Switch
                  checked={item.enabled}
                  onCheckedChange={(enabled) => onToggle(item.id, enabled)}
                  aria-label={`Remove ${item.name}`}
                />
              </div>
            ))}
          </section>

          {customPackages.length > 0 && (
            <section className="data-panel">
              <div className="section-label">Custom rules</div>
              {customPackages.map((name) => (
                <div className="data-row custom-package-grid" key={name}>
                  <code className="truncate">{name}</code>
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={() => onRemoveCustom(name)}
                    aria-label={`Remove ${name} rule`}
                  >
                    <Trash2 className="size-4" />
                  </Button>
                </div>
              ))}
            </section>
          )}
        </>
      ) : (
        <section className="data-panel">
          <div className="data-header installed-grid">
            <span>Package</span>
            <span>Publisher</span>
            <span>Version</span>
            <span />
          </div>
          {filteredInstalled.map((item) => {
            const managed = managedNames.has(item.name.toLowerCase());
            return (
              <div className="data-row installed-grid" key={item.fullName}>
                <div className="min-w-0">
                  <div className="row-primary truncate">{item.name}</div>
                  <div className="row-secondary truncate">{item.fullName}</div>
                </div>
                <span className="row-secondary truncate">{item.publisher}</span>
                <code>{item.version}</code>
                <Button
                  variant="secondary"
                  size="sm"
                  disabled={managed || !item.removable}
                  onClick={() => onAddCustom(item.name)}
                >
                  {managed ? "Managed" : <><Plus className="size-3.5" /> Block</>}
                </Button>
              </div>
            );
          })}
        </section>
      )}
    </div>
  );
}
