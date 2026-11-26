import { useCallback, useEffect, useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import { AlertCircle, AlertTriangle, LoaderCircle, X } from "lucide-react";
import { api } from "@/api";
import markUrl from "@/assets/mark.svg";
import { AppShell } from "@/components/AppShell";
import { ConfirmDialog, type ConfirmRequest } from "@/components/ConfirmDialog";
import { UpdateDialog } from "@/components/UpdateDialog";
import { Button } from "@/components/ui/button";
import { useAppData } from "@/hooks/useAppData";
import { useDesktopUpdate } from "@/hooks/useDesktopUpdate";
import { useResource } from "@/hooks/useResource";
import { errorMessage } from "@/lib/utils";
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
  InstalledPackage,
  NewProcessRule,
  RuntimeSettings,
  Section,
  Snapshot,
} from "@/types";

const PROFILE_FILTERS = [{ name: "JSON", extensions: ["json"] }];

function setCatalogEnabled(
  snapshot: Snapshot,
  section: CatalogSection,
  ids: string[],
  enabled: boolean,
): Snapshot {
  const selected = new Set(ids);
  const toggle = <T extends { id: string; enabled: boolean }>(items: T[]) =>
    items.map((item) => (selected.has(item.id) ? { ...item, enabled } : item));
  switch (section) {
    case "policies":
      return { ...snapshot, policies: toggle(snapshot.policies) };
    case "packages":
      return { ...snapshot, packages: toggle(snapshot.packages) };
    case "networkBlocks":
      return { ...snapshot, networkBlocks: toggle(snapshot.networkBlocks) };
    case "scheduledTasks":
      return { ...snapshot, scheduledTasks: toggle(snapshot.scheduledTasks) };
  }
}

function setPolicyChoice(snapshot: Snapshot, id: string, choice: string): Snapshot {
  return {
    ...snapshot,
    policies: snapshot.policies.map((item) =>
      item.id === id
        ? {
            ...item,
            choice,
            wanted: item.choices.find((option) => option.id === choice)?.label ?? item.wanted,
          }
        : item,
    ),
  };
}

function updateSettings(
  snapshot: Snapshot,
  update: (settings: Snapshot["settings"]) => Partial<Snapshot["settings"]>,
): Snapshot {
  return { ...snapshot, settings: { ...snapshot.settings, ...update(snapshot.settings) } };
}

export default function App() {
  const { snapshot, setSnapshot, loading, error, setError, pending, loadSnapshot, mutate } =
    useAppData();
  const updater = useDesktopUpdate();
  const [section, setSection] = useState<Section>("overview");
  const [confirmation, setConfirmation] = useState<ConfirmRequest | null>(null);
  const [dismissedIssues, setDismissedIssues] = useState("");
  const reportError = useCallback((caught: unknown) => setError(errorMessage(caught)), [setError]);
  const autostarts = useResource(api.listAutostarts, reportError);
  const processes = useResource(api.listProcesses, reportError);

  useEffect(() => {
    if (section === "autostart" && !autostarts.attempted && !autostarts.loading) {
      void autostarts.reload();
    }
  }, [autostarts, section]);

  function toggleCatalog(catalog: CatalogSection, ids: string[], enabled: boolean) {
    if (ids.length === 0) return;
    void mutate(
      ids.length === 1 ? `${catalog}:${ids[0]}` : `${catalog}:bulk`,
      () => api.setCatalogItems(catalog, ids, enabled),
      (current) => setCatalogEnabled(current, catalog, ids, enabled),
    );
  }

  function choosePolicyOption(id: string, choice: string) {
    void mutate(
      `choice:${id}`,
      () => api.setPolicyChoice(id, choice),
      (current) => setPolicyChoice(current, id, choice),
    );
  }

  function enforceNow() {
    void mutate("enforce", api.enforceNow);
  }

  function setEnforcementEnabled(enabled: boolean) {
    const apply = () =>
      void mutate(
        "enforcement",
        () => api.setEnforcementEnabled(enabled),
        (current) => updateSettings(current, () => ({ enforcementEnabled: enabled })),
      );
    if (!enabled) {
      apply();
      return;
    }
    setConfirmation({
      title: "Enable enforcement?",
      description:
        "Selected changes are applied now, including app removals.",
      confirmLabel: "Enable",
      onConfirm: apply,
    });
  }

  function removeInstalledApp(item: InstalledPackage) {
    setConfirmation({
      title: `Remove ${item.name}?`,
      description: snapshot?.settings.enforcementEnabled
        ? "The app is removed for all users and stays blocked."
        : "The app is removed for all users once enforcement is enabled.",
      confirmLabel: "Remove",
      onConfirm: () => void mutate(`custom:${item.name}`, () => api.addCustomPackage(item.name)),
    });
  }

  function removeCustomPackage(name: string) {
    void mutate(
      `custom:${name}`,
      () => api.removeCustomPackage(name),
      (current) =>
        updateSettings(current, (settings) => ({
          customPackages: settings.customPackages.filter((item) => item !== name),
        })),
    );
  }

  async function blockAutostart(entry: AutostartEntry) {
    const succeeded = await mutate(`autostart:${entry.id}`, () => api.blockAutostart(entry));
    if (succeeded) void autostarts.reload();
  }

  function removeAutostartRule(id: string) {
    void mutate(
      `autostart-rule:${id}`,
      () => api.removeAutostartRule(id),
      (current) =>
        updateSettings(current, (settings) => ({
          blockedAutostarts: settings.blockedAutostarts.filter((rule) => rule.id !== id),
        })),
    );
  }

  function setProcessRuleEnabled(id: string, enabled: boolean) {
    void mutate(
      `process:${id}`,
      () => api.setProcessRuleEnabled(id, enabled),
      (current) =>
        updateSettings(current, (settings) => ({
          processRules: settings.processRules.map((rule) =>
            rule.id === id ? { ...rule, enabled } : rule,
          ),
        })),
    );
  }

  function setProcessRuleNotificationsMuted(id: string, muted: boolean) {
    void mutate(
      `notify:${id}`,
      () => api.setProcessRuleNotificationsMuted(id, muted),
      (current) =>
        updateSettings(current, (settings) => ({
          mutedProcessNotifications: muted
            ? [...settings.mutedProcessNotifications.filter((ruleId) => ruleId !== id), id]
            : settings.mutedProcessNotifications.filter((ruleId) => ruleId !== id),
        })),
    );
  }

  function addProcessRule(rule: NewProcessRule) {
    void mutate(`process-add:${rule.executableName}`, () => api.addProcessRule(rule));
  }

  function removeProcessRule(id: string) {
    void mutate(
      `process-remove:${id}`,
      () => api.removeProcessRule(id),
      (current) =>
        updateSettings(current, (settings) => ({
          processRules: settings.processRules.filter((rule) => rule.id !== id),
        })),
    );
  }

  function saveRuntimeSettings(settings: RuntimeSettings) {
    return mutate("settings", () => api.updateRuntimeSettings(settings));
  }

  async function exportProfile() {
    const path = await save({
      defaultPath: "prep-my-windows-policy.json",
      filters: PROFILE_FILTERS,
    });
    if (path) void mutate("profile", () => api.exportProfile(path));
  }

  async function importProfile() {
    const path = await open({ multiple: false, directory: false, filters: PROFILE_FILTERS });
    if (typeof path === "string") void mutate("profile", () => api.importProfile(path));
  }

  function resetProfile() {
    setConfirmation({
      title: "Reset profile?",
      description: snapshot?.settings.enforcementEnabled
        ? "All rules return to the built-in profile and are enforced immediately. Windows settings already changed are not restored."
        : "All rules return to the built-in profile. Windows settings already changed are not restored.",
      confirmLabel: "Reset",
      onConfirm: () => void mutate("profile", api.resetProfile),
    });
  }

  async function clearActivity() {
    try {
      await api.clearActivity();
      setSnapshot((current) => (current ? { ...current, activity: [] } : current));
    } catch (caught) {
      reportError(caught);
    }
  }

  if (loading && !snapshot) {
    return (
      <div className="startup-screen">
        <img className="brand-mark large" src={markUrl} alt="" />
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

  const issuesKey = snapshot.issues.join("\n");
  const showIssues = snapshot.issues.length > 0 && issuesKey !== dismissedIssues;

  return (
    <AppShell
      section={section}
      setSection={setSection}
      system={snapshot.system}
      status={snapshot.status}
      enforcementEnabled={snapshot.settings.enforcementEnabled}
      enforcing={pending.has("enforce")}
      switchingEnforcement={pending.has("enforcement")}
      onEnforce={enforceNow}
      onSetEnforcementEnabled={setEnforcementEnabled}
      updater={updater}
    >
      {error && (
        <div className="banner error" role="alert">
          <AlertCircle className="mt-0.5 size-4 shrink-0" />
          <span className="banner-text">{error}</span>
          <button onClick={() => setError(null)} aria-label="Dismiss">
            <X className="size-4" />
          </button>
        </div>
      )}
      {showIssues && (
        <div className="banner warning" role="status">
          <AlertTriangle className="mt-0.5 size-4 shrink-0" />
          <span className="banner-text">{issuesKey}</span>
          <button onClick={() => setDismissedIssues(issuesKey)} aria-label="Dismiss">
            <X className="size-4" />
          </button>
        </div>
      )}
      {section === "overview" && <OverviewView snapshot={snapshot} onNavigate={setSection} />}
      {section === "policy" && (
        <PolicyView
          policies={snapshot.policies}
          tasks={snapshot.scheduledTasks}
          pending={pending}
          onTogglePolicies={(ids, enabled) => toggleCatalog("policies", ids, enabled)}
          onToggleTasks={(ids, enabled) => toggleCatalog("scheduledTasks", ids, enabled)}
          onChoosePolicyOption={choosePolicyOption}
        />
      )}
      {section === "apps" && (
        <AppsView
          rules={snapshot.packages}
          installed={snapshot.installedPackages}
          customPackages={snapshot.settings.customPackages}
          pending={pending}
          onToggle={(ids, enabled) => toggleCatalog("packages", ids, enabled)}
          onAddCustom={removeInstalledApp}
          onRemoveCustom={removeCustomPackage}
        />
      )}
      {section === "autostart" && (
        <AutostartView
          entries={autostarts.items}
          blocked={snapshot.settings.blockedAutostarts}
          loading={autostarts.loading}
          pending={pending}
          onRefresh={autostarts.reload}
          onBlock={blockAutostart}
          onRemoveRule={removeAutostartRule}
        />
      )}
      {section === "processes" && (
        <ProcessesView
          rules={snapshot.settings.processRules}
          mutedRuleIds={snapshot.settings.mutedProcessNotifications}
          processes={processes.items}
          loadingProcesses={processes.loading}
          pending={pending}
          onLoadProcesses={processes.reload}
          onToggle={setProcessRuleEnabled}
          onNotificationsMuted={setProcessRuleNotificationsMuted}
          onAdd={addProcessRule}
          onRemove={removeProcessRule}
        />
      )}
      {section === "network" && (
        <NetworkView
          items={snapshot.networkBlocks}
          pending={pending}
          onToggle={(ids, enabled) => toggleCatalog("networkBlocks", ids, enabled)}
        />
      )}
      {section === "activity" && (
        <ActivityView activity={snapshot.activity} onClear={clearActivity} />
      )}
      {section === "settings" && (
        <SettingsView
          settings={snapshot.settings}
          system={snapshot.system}
          pending={pending}
          onSave={saveRuntimeSettings}
          onExport={exportProfile}
          onImport={importProfile}
          onReset={resetProfile}
          updater={updater}
        />
      )}
      <ConfirmDialog request={confirmation} onClose={() => setConfirmation(null)} />
      <UpdateDialog updater={updater} />
    </AppShell>
  );
}
