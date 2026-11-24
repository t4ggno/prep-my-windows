import { useMemo, useState } from "react";
import { LoaderCircle, Trash2 } from "lucide-react";
import {
  BulkToggle,
  EmptyState,
  RuleSwitch,
  SearchField,
  SegmentedControl,
  StateDot,
  ViewHeader,
} from "@/components/common";
import { Button } from "@/components/ui/button";
import { matchesQuery } from "@/lib/utils";
import type { InstalledPackage, PackageItem } from "@/types";

type AppsTab = "rules" | "installed";

function packageState(item: PackageItem) {
  if (!item.installed) return "Not installed";
  return item.provisioned ? "Provisioned" : "Installed";
}

export function AppsView({
  rules,
  installed,
  customPackages,
  pending,
  onToggle,
  onAddCustom,
  onRemoveCustom,
}: {
  rules: PackageItem[];
  installed: InstalledPackage[];
  customPackages: string[];
  pending: ReadonlySet<string>;
  onToggle: (ids: string[], enabled: boolean) => void;
  onAddCustom: (item: InstalledPackage) => void;
  onRemoveCustom: (name: string) => void;
}) {
  const [tab, setTab] = useState<AppsTab>("rules");
  const [query, setQuery] = useState("");
  const managedNames = useMemo(
    () =>
      new Set([
        ...rules.map((item) => item.packageName.toLowerCase()),
        ...customPackages.map((item) => item.toLowerCase()),
      ]),
    [customPackages, rules],
  );
  const filteredRules = rules.filter((item) => matchesQuery(query, item.name, item.packageName));
  const filteredCustom = customPackages.filter((name) => matchesQuery(query, name));
  const filteredInstalled = installed.filter((item) =>
    matchesQuery(query, item.name, item.publisher, item.fullName),
  );

  return (
    <div className="view-stack">
      <ViewHeader title="Apps" actions={<SearchField value={query} onChange={setQuery} />} />
      <div className="flex items-center justify-between gap-3">
        <SegmentedControl
          value={tab}
          onChange={setTab}
          options={[
            { id: "rules", label: "Removal rules", count: rules.length + customPackages.length },
            { id: "installed", label: "Installed", count: installed.length },
          ]}
        />
        {tab === "rules" && (
          <BulkToggle
            items={filteredRules}
            pending={pending.has("packages:bulk")}
            onChange={(enabled) =>
              onToggle(
                filteredRules.filter((item) => item.enabled !== enabled).map((item) => item.id),
                enabled,
              )
            }
          />
        )}
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
                  {packageState(item)}
                </span>
                <RuleSwitch
                  checked={item.enabled}
                  pending={pending.has(`packages:${item.id}`)}
                  onCheckedChange={(enabled) => onToggle([item.id], enabled)}
                  label={`Remove ${item.name}`}
                />
              </div>
            ))}
            {filteredRules.length === 0 && <EmptyState>No matching apps</EmptyState>}
          </section>

          {filteredCustom.length > 0 && (
            <section className="data-panel">
              <div className="section-label">Custom rules</div>
              {filteredCustom.map((name) => (
                <div className="data-row custom-package-grid" key={name}>
                  <code className="truncate">{name}</code>
                  <Button
                    variant="ghost"
                    size="icon"
                    disabled={pending.has(`custom:${name}`)}
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
            const removing = pending.has(`custom:${item.name}`);
            return (
              <div className="data-row installed-grid" key={item.fullName}>
                <div className="min-w-0">
                  <div className="row-primary truncate">{item.name}</div>
                  <div className="row-secondary truncate" title={item.fullName}>
                    {item.provisioned ? `Provisioned · ${item.fullName}` : item.fullName}
                  </div>
                </div>
                <span className="row-secondary truncate" title={item.publisher}>
                  {item.publisher}
                </span>
                <code className="truncate">{item.version}</code>
                <Button
                  variant="secondary"
                  size="sm"
                  disabled={managed || removing || !item.removable}
                  onClick={() => onAddCustom(item)}
                >
                  {removing ? (
                    <LoaderCircle className="size-3.5 animate-spin" />
                  ) : managed ? (
                    "Managed"
                  ) : item.removable ? (
                    <>
                      <Trash2 className="size-3.5" /> Remove
                    </>
                  ) : (
                    "System app"
                  )}
                </Button>
              </div>
            );
          })}
          {filteredInstalled.length === 0 && <EmptyState>No matching apps</EmptyState>}
        </section>
      )}
    </div>
  );
}
