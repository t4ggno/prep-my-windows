import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { Check, FileUp, ListPlus, RefreshCw, Trash2 } from "lucide-react";
import {
  EmptyState,
  LoadingState,
  RuleSwitch,
  SearchField,
  ViewHeader,
} from "@/components/common";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cn, matchesQuery } from "@/lib/utils";
import type { NewProcessRule, ProcessInfo, ProcessRule } from "@/types";

function executableName(path: string) {
  return path.split(/[\\/]/).pop() ?? path;
}

function ruleFromExecutable(name: string, path: string | null): NewProcessRule {
  return { name: name.replace(/\.exe$/i, ""), executableName: name, executablePath: path };
}

function coversProcess(rule: ProcessRule, process: ProcessInfo) {
  return (
    rule.executableName.toLowerCase() === process.name.toLowerCase() &&
    (!rule.executablePath ||
      rule.executablePath.toLowerCase() === process.executablePath.toLowerCase())
  );
}

export function ProcessesView({
  rules,
  mutedRuleIds,
  processes,
  loadingProcesses,
  pending,
  onLoadProcesses,
  onToggle,
  onNotificationsMuted,
  onAdd,
  onRemove,
}: {
  rules: ProcessRule[];
  mutedRuleIds: string[];
  processes: ProcessInfo[];
  loadingProcesses: boolean;
  pending: ReadonlySet<string>;
  onLoadProcesses: () => void;
  onToggle: (id: string, enabled: boolean) => void;
  onNotificationsMuted: (id: string, muted: boolean) => void;
  onAdd: (process: NewProcessRule) => void;
  onRemove: (id: string) => void;
}) {
  const [query, setQuery] = useState("");
  const [pickerOpen, setPickerOpen] = useState(false);
  const [pickerQuery, setPickerQuery] = useState("");
  const muted = new Set(mutedRuleIds);
  const filteredRules = rules.filter((rule) =>
    matchesQuery(query, rule.name, rule.executableName, rule.executablePath),
  );
  const filteredProcesses = processes.filter((process) =>
    matchesQuery(pickerQuery, process.name, process.executablePath),
  );

  async function chooseExecutable() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Applications", extensions: ["exe"] }],
    });
    if (typeof selected !== "string") return;
    onAdd(ruleFromExecutable(executableName(selected), selected));
  }

  function openPicker() {
    setPickerQuery("");
    setPickerOpen(true);
    onLoadProcesses();
  }

  return (
    <div className="view-stack">
      <ViewHeader
        title="Process rules"
        actions={
          <>
            <SearchField value={query} onChange={setQuery} />
            <Button variant="secondary" size="sm" onClick={openPicker}>
              <ListPlus className="size-4" /> Running process
            </Button>
            <Button variant="secondary" size="sm" onClick={chooseExecutable}>
              <FileUp className="size-4" /> Choose .exe
            </Button>
          </>
        }
      />

      <section className="data-panel">
        <div className="data-header process-grid">
          <span>Rule</span>
          <span>Executable</span>
          <span>Notify</span>
          <span>Stop</span>
          <span />
        </div>
        {filteredRules.map((rule) => (
          <div className="data-row process-grid" key={rule.id}>
            <div className="min-w-0">
              <div className="row-primary truncate">{rule.name}</div>
              <div className="row-secondary">{rule.builtIn ? "Default profile" : "Custom"}</div>
            </div>
            <div className="min-w-0">
              <code className="block truncate">{rule.executableName}</code>
              {rule.executablePath && (
                <div className="row-secondary truncate" title={rule.executablePath}>
                  {rule.executablePath}
                </div>
              )}
            </div>
            <RuleSwitch
              checked={!muted.has(rule.id)}
              pending={pending.has(`notify:${rule.id}`)}
              disabled={!rule.enabled}
              onCheckedChange={(notify) => onNotificationsMuted(rule.id, !notify)}
              label={`Notify when ${rule.name} is stopped`}
            />
            <RuleSwitch
              checked={rule.enabled}
              pending={pending.has(`process:${rule.id}`)}
              onCheckedChange={(enabled) => onToggle(rule.id, enabled)}
              label={`Stop ${rule.name}`}
            />
            {rule.builtIn ? (
              <span />
            ) : (
              <Button
                variant="ghost"
                size="icon"
                disabled={pending.has(`process-remove:${rule.id}`)}
                onClick={() => onRemove(rule.id)}
                aria-label={`Remove ${rule.name}`}
              >
                <Trash2 className="size-4" />
              </Button>
            )}
          </div>
        ))}
        {filteredRules.length === 0 && <EmptyState>No matching rules</EmptyState>}
      </section>

      <Dialog open={pickerOpen} onOpenChange={setPickerOpen}>
        <DialogContent aria-describedby={undefined}>
          <div className="dialog-header">
            <DialogTitle>Running processes</DialogTitle>
          </div>
          <div className="dialog-tools">
            <SearchField value={pickerQuery} onChange={setPickerQuery} className="w-full" />
            <Button
              variant="secondary"
              size="icon"
              onClick={onLoadProcesses}
              disabled={loadingProcesses}
              aria-label="Refresh"
            >
              <RefreshCw className={cn("size-4", loadingProcesses && "animate-spin")} />
            </Button>
          </div>
          <ScrollArea className="h-[520px]">
            <div className="process-picker-list">
              {filteredProcesses.map((process) => {
                const covered = rules.some((rule) => coversProcess(rule, process));
                return (
                  <button
                    key={`${process.name}-${process.executablePath}`}
                    disabled={covered}
                    onClick={() => {
                      onAdd(ruleFromExecutable(process.name, process.executablePath || null));
                      setPickerOpen(false);
                    }}
                  >
                    <div className="min-w-0 flex-1 text-left">
                      <div className="row-primary truncate">{process.name}</div>
                      <div className="row-secondary truncate">
                        {process.executablePath || "Path unavailable"}
                      </div>
                    </div>
                    {covered ? (
                      <Check className="size-4 text-emerald-400" aria-label="Rule exists" />
                    ) : (
                      process.instances > 1 && <code>×{process.instances}</code>
                    )}
                  </button>
                );
              })}
              {loadingProcesses && processes.length === 0 && <LoadingState />}
              {!loadingProcesses && filteredProcesses.length === 0 && (
                <EmptyState>No processes found</EmptyState>
              )}
            </div>
          </ScrollArea>
        </DialogContent>
      </Dialog>
    </div>
  );
}
