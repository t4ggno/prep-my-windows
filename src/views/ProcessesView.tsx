import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { FileUp, ListPlus, RefreshCw, Trash2 } from "lucide-react";
import { EmptyState, SearchField, ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Switch } from "@/components/ui/switch";
import { cn } from "@/lib/utils";
import type { ProcessInfo, ProcessRule } from "@/types";

export function ProcessesView({
  rules,
  mutedRuleIds,
  processes,
  loadingProcesses,
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
  onLoadProcesses: () => void;
  onToggle: (id: string, enabled: boolean) => void;
  onNotificationsMuted: (id: string, muted: boolean) => void;
  onAdd: (process: { name: string; executableName: string; executablePath: string | null }) => void;
  onRemove: (id: string) => void;
}) {
  const [query, setQuery] = useState("");
  const [pickerOpen, setPickerOpen] = useState(false);
  const [pickerQuery, setPickerQuery] = useState("");
  const filteredRules = rules.filter((rule) => {
    const value = query.trim().toLowerCase();
    return (
      !value ||
      rule.name.toLowerCase().includes(value) ||
      rule.executableName.toLowerCase().includes(value) ||
      rule.executablePath?.toLowerCase().includes(value)
    );
  });
  const filteredProcesses = processes.filter((process) => {
    const value = pickerQuery.trim().toLowerCase();
    return (
      !value ||
      process.name.toLowerCase().includes(value) ||
      process.executablePath.toLowerCase().includes(value)
    );
  });

  async function chooseExecutable() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Applications", extensions: ["exe"] }],
    });
    if (typeof selected !== "string") return;
    const executableName = selected.split(/[\\/]/).pop() ?? selected;
    onAdd({ name: executableName.replace(/\.exe$/i, ""), executableName, executablePath: selected });
  }

  function openPicker() {
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
              {rule.executablePath && <div className="row-secondary truncate">{rule.executablePath}</div>}
            </div>
            <Switch
              checked={!mutedRuleIds.includes(rule.id)}
              onCheckedChange={(notify) => onNotificationsMuted(rule.id, !notify)}
              aria-label={`Notify when ${rule.name} is stopped`}
            />
            <Switch
              checked={rule.enabled}
              onCheckedChange={(enabled) => onToggle(rule.id, enabled)}
              aria-label={`Block ${rule.name}`}
            />
            <Button
              variant="ghost"
              size="icon"
              onClick={() => onRemove(rule.id)}
              aria-label={`Remove ${rule.name}`}
            >
              <Trash2 className="size-4" />
            </Button>
          </div>
        ))}
      </section>

      <Dialog open={pickerOpen} onOpenChange={setPickerOpen}>
        <DialogContent aria-describedby={undefined}>
          <div className="dialog-header">
            <DialogTitle>Running processes</DialogTitle>
          </div>
          <div className="dialog-tools">
            <SearchField value={pickerQuery} onChange={setPickerQuery} className="w-full" />
            <Button variant="secondary" size="icon" onClick={onLoadProcesses} disabled={loadingProcesses}>
              <RefreshCw className={cn("size-4", loadingProcesses && "animate-spin")} />
            </Button>
          </div>
          <ScrollArea className="h-[520px]">
            <div className="process-picker-list">
              {filteredProcesses.map((process) => (
                <button
                  key={`${process.pid}-${process.executablePath}`}
                  onClick={() => {
                    onAdd({
                      name: process.name.replace(/\.exe$/i, ""),
                      executableName: process.name,
                      executablePath: process.executablePath || null,
                    });
                    setPickerOpen(false);
                  }}
                >
                  <div className="min-w-0 flex-1 text-left">
                    <div className="row-primary truncate">{process.name}</div>
                    <div className="row-secondary truncate">{process.executablePath || "Path unavailable"}</div>
                  </div>
                  <code>{process.pid}</code>
                </button>
              ))}
              {!loadingProcesses && !filteredProcesses.length && <EmptyState>No processes found</EmptyState>}
            </div>
          </ScrollArea>
        </DialogContent>
      </Dialog>
    </div>
  );
}
