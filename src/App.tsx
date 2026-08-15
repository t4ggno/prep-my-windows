import { useCallback, useEffect, useRef, useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import { AlertCircle, LoaderCircle, X } from "lucide-react";
import { api } from "@/api";
import { AppShell } from "@/components/AppShell";
import { Button } from "@/components/ui/button";
import { ActivityView } from "@/views/ActivityView";
import { AppsView } from "@/views/AppsView";
import { AutostartView } from "@/views/AutostartView";
import { NetworkView } from "@/views/NetworkView";
import { OverviewView } from "@/views/OverviewView";
import { PolicyView } from "@/views/PolicyView";
import { ProcessesView } from "@/views/ProcessesView";
import { SettingsView } from "@/views/SettingsView";
import type {
  AutostartEntry,
  CatalogSection,
  ProcessInfo,
  RuntimeSettings,
  Section,
  Snapshot,
} from "@/types";

function errorMessage(error: unknown) {
  if (typeof error === "string") return error;
  if (error instanceof Error) return error.message;
  return "The operation failed";
}

export default function App() {
  const [section, setSection] = useState<Section>("overview");
  const [snapshot, setSnapshot] = useState<Snapshot | null>(null);
  const [loading, setLoading] = useState(true);
  const [enforcing, setEnforcing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [autostarts, setAutostarts] = useState<AutostartEntry[]>([]);
  const [loadingAutostarts, setLoadingAutostarts] = useState(false);
  const [autostartsLoaded, setAutostartsLoaded] = useState(false);
  const [processes, setProcesses] = useState<ProcessInfo[]>([]);
  const [loadingProcesses, setLoadingProcesses] = useState(false);
  const snapshotRepairTotal = useRef<number | null>(null);
  const snapshotRequest = useRef(0);

  const loadSnapshot = useCallback(async (showLoading = false) => {
    const request = ++snapshotRequest.current;
    if (showLoading) setLoading(true);
    try {
      const next = await api.snapshot();
      if (request !== snapshotRequest.current) return;
      snapshotRepairTotal.current = next.status.repairedTotal;
      setSnapshot(next);
      setError(null);
    } catch (caught) {
      if (request !== snapshotRequest.current) return;
      setError(errorMessage(caught));
    } finally {
      if (request === snapshotRequest.current) setLoading(false);
    }
  }, []);

  const loadAutostarts = useCallback(async () => {
    setLoadingAutostarts(true);
    try {
      setAutostarts(await api.listAutostarts());
      setAutostartsLoaded(true);
      setError(null);
    } catch (caught) {
      setError(errorMessage(caught));
    } finally {
      setLoadingAutostarts(false);
    }
  }, []);

  const loadProcesses = useCallback(async () => {
    setLoadingProcesses(true);
    try {
      setProcesses(await api.listProcesses());
      setError(null);
    } catch (caught) {
      setError(errorMessage(caught));
    } finally {
      setLoadingProcesses(false);
    }
  }, []);

  useEffect(() => {
    void loadSnapshot(true);
  }, [loadSnapshot]);

  useEffect(() => {
    if (section === "autostart" && !autostartsLoaded) {
      void loadAutostarts();
    }
  }, [autostartsLoaded, loadAutostarts, section]);

  useEffect(() => {
    const interval = window.setInterval(async () => {
      try {
        const live = await api.liveState();
        if (
          snapshotRepairTotal.current !== null &&
          live.status.repairedTotal !== snapshotRepairTotal.current
        ) {
          await loadSnapshot();
          return;
        }
        setSnapshot((current) =>
          current
            ? { ...current, status: live.status, activity: live.activity }
            : current,
        );
      } catch {
        return;
      }
    }, 4000);
    return () => window.clearInterval(interval);
  }, [loadSnapshot]);

  async function toggleCatalog(section: CatalogSection, id: string, enabled: boolean) {
    snapshotRequest.current += 1;
    setSnapshot((current) => {
      if (!current) return current;
      if (section === "policies") {
        return {
          ...current,
          policies: current.policies.map((item) =>
            item.id === id ? { ...item, enabled } : item,
          ),
        };
      }
      if (section === "packages") {
        return {
          ...current,
          packages: current.packages.map((item) =>
            item.id === id ? { ...item, enabled } : item,
          ),
        };
      }
      if (section === "networkBlocks") {
        return {
          ...current,
          networkBlocks: current.networkBlocks.map((item) =>
            item.id === id ? { ...item, enabled } : item,
          ),
        };
      }
      return {
        ...current,
        scheduledTasks: current.scheduledTasks.map((item) =>
          item.id === id ? { ...item, enabled } : item,
        ),
      };
    });
    try {
      await api.setCatalogItem(section, id, enabled);
      await loadSnapshot();
      setError(null);
    } catch (caught) {
      setError(errorMessage(caught));
      await loadSnapshot();
    }
  }

  async function enforceNow() {
    setEnforcing(true);
    try {
      await api.enforceNow();
      setError(null);
    } catch (caught) {
      setError(errorMessage(caught));
    } finally {
      await loadSnapshot();
      setEnforcing(false);
    }
  }

  async function addCustomPackage(name: string) {
    setEnforcing(true);
    try {
      await api.addCustomPackage(name);
      await api.enforceNow();
      setError(null);
    } catch (caught) {
      setError(errorMessage(caught));
    } finally {
      await loadSnapshot();
      setEnforcing(false);
    }
  }

  async function removeCustomPackage(name: string) {
    try {
      await api.removeCustomPackage(name);
      await loadSnapshot();
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function blockAutostart(entry: AutostartEntry) {
    try {
      await api.blockAutostart(entry);
      setAutostarts((current) => current.filter((item) => item.id !== entry.id));
      await loadSnapshot();
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function removeAutostartRule(id: string) {
    try {
      await api.removeAutostartRule(id);
      await loadSnapshot();
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function setProcessRuleEnabled(id: string, enabled: boolean) {
    snapshotRequest.current += 1;
    setSnapshot((current) =>
      current
        ? {
            ...current,
            settings: {
              ...current.settings,
              processRules: current.settings.processRules.map((rule) =>
                rule.id === id ? { ...rule, enabled } : rule,
              ),
            },
          }
        : current,
    );
    try {
      await api.setProcessRuleEnabled(id, enabled);
      await loadSnapshot();
      setError(null);
    } catch (caught) {
      setError(errorMessage(caught));
      await loadSnapshot();
    }
  }

  async function addProcessRule(process: {
    name: string;
    executableName: string;
    executablePath: string | null;
  }) {
    try {
      await api.addProcessRule(process);
      await loadSnapshot();
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function removeProcessRule(id: string) {
    try {
      await api.removeProcessRule(id);
      await loadSnapshot();
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function saveRuntimeSettings(settings: RuntimeSettings) {
    try {
      await api.updateRuntimeSettings(settings);
      await loadSnapshot();
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function exportProfile() {
    const path = await save({
      defaultPath: "prep-my-windows-policy.json",
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (!path) return;
    try {
      await api.exportProfile(path);
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function importProfile() {
    const path = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (typeof path !== "string") return;
    try {
      await api.importProfile(path);
      await loadSnapshot();
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function resetProfile() {
    try {
      await api.resetProfile();
      await loadSnapshot();
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  async function clearActivity() {
    try {
      await api.clearActivity();
      setSnapshot((current) => (current ? { ...current, activity: [] } : current));
    } catch (caught) {
      setError(errorMessage(caught));
    }
  }

  if (loading && !snapshot) {
    return (
      <div className="startup-screen">
        <div className="brand-mark large"><span /><span /><span /><span /></div>
        <LoaderCircle className="size-5 animate-spin text-sky-400" />
      </div>
    );
  }

  if (!snapshot) {
    return (
      <div className="startup-screen gap-4">
        <AlertCircle className="size-8 text-red-400" />
        <div className="max-w-md text-center text-sm text-zinc-400">{error}</div>
        <Button onClick={() => loadSnapshot(true)}>Retry</Button>
      </div>
    );
  }

  return (
    <AppShell
      section={section}
      setSection={setSection}
      system={snapshot.system}
      status={snapshot.status}
      enforcing={enforcing}
      onEnforce={enforceNow}
    >
      {error && (
        <div className="error-banner">
          <AlertCircle className="size-4 shrink-0" />
          <span className="min-w-0 flex-1 truncate">{error}</span>
          <button onClick={() => setError(null)} aria-label="Dismiss error">
            <X className="size-4" />
          </button>
        </div>
      )}
      {section === "overview" && <OverviewView snapshot={snapshot} />}
      {section === "policy" && (
        <PolicyView
          policies={snapshot.policies}
          tasks={snapshot.scheduledTasks}
          onTogglePolicy={(id, enabled) => toggleCatalog("policies", id, enabled)}
          onToggleTask={(id, enabled) => toggleCatalog("scheduledTasks", id, enabled)}
        />
      )}
      {section === "apps" && (
        <AppsView
          rules={snapshot.packages}
          installed={snapshot.installedPackages}
          customPackages={snapshot.settings.customPackages}
          onToggle={(id, enabled) => toggleCatalog("packages", id, enabled)}
          onAddCustom={addCustomPackage}
          onRemoveCustom={removeCustomPackage}
        />
      )}
      {section === "autostart" && (
        <AutostartView
          entries={autostarts}
          blocked={snapshot.settings.blockedAutostarts}
          loading={loadingAutostarts}
          onRefresh={loadAutostarts}
          onBlock={blockAutostart}
          onRemoveRule={removeAutostartRule}
        />
      )}
      {section === "processes" && (
        <ProcessesView
          rules={snapshot.settings.processRules}
          processes={processes}
          loadingProcesses={loadingProcesses}
          onLoadProcesses={loadProcesses}
          onToggle={setProcessRuleEnabled}
          onAdd={addProcessRule}
          onRemove={removeProcessRule}
        />
      )}
      {section === "network" && (
        <NetworkView
          items={snapshot.networkBlocks}
          onToggle={(id, enabled) => toggleCatalog("networkBlocks", id, enabled)}
        />
      )}
      {section === "activity" && (
        <ActivityView activity={snapshot.activity} onClear={clearActivity} />
      )}
      {section === "settings" && (
        <SettingsView
          settings={snapshot.settings}
          system={snapshot.system}
          onSave={saveRuntimeSettings}
          onExport={exportProfile}
          onImport={importProfile}
          onReset={resetProfile}
        />
      )}
    </AppShell>
  );
}
