import { useEffect, useState } from "react";
import { getVersion } from "@tauri-apps/api/app";
import { Download, LoaderCircle, RefreshCw, RotateCcw, Upload } from "lucide-react";
import { ActiveHoursPicker } from "@/components/ActiveHoursPicker";
import { ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import type { DesktopUpdate } from "@/hooks/useDesktopUpdate";
import type { AppSettings, RuntimeSettings, SystemInfo } from "@/types";

function savedSettings(settings: AppSettings): RuntimeSettings {
  return {
    startWithWindows: settings.startWithWindows,
    activeHoursStart: settings.activeHoursStart,
    activeHoursEnd: settings.activeHoursEnd,
  };
}

export function SettingsView({
  settings,
  system,
  pending,
  onSave,
  onExport,
  onImport,
  onReset,
  updater,
}: {
  settings: AppSettings;
  system: SystemInfo;
  pending: ReadonlySet<string>;
  onSave: (settings: RuntimeSettings) => Promise<boolean>;
  onExport: () => void;
  onImport: () => void;
  onReset: () => void;
  updater: DesktopUpdate;
}) {
  const saved = savedSettings(settings);
  const [draft, setDraft] = useState<RuntimeSettings | null>(null);
  const [version, setVersion] = useState<string | null>(null);
  const values = draft ?? saved;
  const dirty =
    draft !== null &&
    (draft.startWithWindows !== saved.startWithWindows ||
      draft.activeHoursStart !== saved.activeHoursStart ||
      draft.activeHoursEnd !== saved.activeHoursEnd);
  const saving = pending.has("settings");
  const profileBusy = pending.has("profile");

  useEffect(() => {
    let disposed = false;
    void getVersion()
      .then((installed) => {
        if (!disposed) setVersion(installed);
      })
      .catch((failure: unknown) => console.error("Could not read the app version", failure));
    return () => {
      disposed = true;
    };
  }, []);

  function update(change: Partial<RuntimeSettings>) {
    setDraft({ ...values, ...change });
  }

  async function save() {
    if (await onSave(values)) setDraft(null);
  }

  return (
    <div className="view-stack settings-view">
      <ViewHeader title="Settings" />
      <section className="settings-card">
        <div className="section-label">Enforcement</div>
        <div className="settings-row">
          <span>Windows Update active hours</span>
          <div className="flex items-center gap-2 text-xs text-zinc-500">
            <ActiveHoursPicker className="w-28" hours={values} edge="start" onChange={update} />
            <span>to</span>
            <ActiveHoursPicker className="w-28" hours={values} edge="end" onChange={update} />
          </div>
        </div>
        <label className="settings-row">
          <span>Start with Windows</span>
          <Switch
            checked={values.startWithWindows}
            onCheckedChange={(startWithWindows) => update({ startWithWindows })}
          />
        </label>
        <div className="settings-actions">
          <Button onClick={save} disabled={!dirty || saving}>
            {saving && <LoaderCircle className="size-4 animate-spin" />}
            Save settings
          </Button>
          {dirty && (
            <Button variant="ghost" onClick={() => setDraft(null)} disabled={saving}>
              Discard
            </Button>
          )}
        </div>
      </section>

      <section className="settings-card">
        <div className="section-label">Profile</div>
        <div className="button-row">
          <Button variant="secondary" onClick={onExport} disabled={profileBusy}>
            <Download className="size-4" /> Export
          </Button>
          <Button variant="secondary" onClick={onImport} disabled={profileBusy}>
            <Upload className="size-4" /> Import
          </Button>
          <Button variant="secondary" onClick={onReset} disabled={profileBusy}>
            {profileBusy ? (
              <LoaderCircle className="size-4 animate-spin" />
            ) : (
              <RotateCcw className="size-4" />
            )}
            Reset profile
          </Button>
        </div>
      </section>

      {updater.supported && (
        <section className="settings-card">
          <div className="section-label">Updates</div>
          <div className="settings-row">
            <span>{version ? `Version ${version}` : "Version"}</span>
            <Button
              variant="secondary"
              onClick={updater.check}
              disabled={updater.phase === "checking"}
            >
              {updater.phase === "checking" ? (
                <LoaderCircle className="size-4 animate-spin" />
              ) : (
                <RefreshCw className="size-4" />
              )}
              Check for updates
            </Button>
          </div>
        </section>
      )}

      <section className="settings-card">
        <div className="section-label">System</div>
        <div className="system-grid">
          <span>Edition</span>
          <strong>{system.productName}</strong>
          <span>Version</span>
          <strong>{system.displayVersion}</strong>
          <span>Build</span>
          <strong>{system.buildNumber}</strong>
          <span>Access</span>
          <strong>{system.isElevated ? "Administrator" : "Standard user"}</strong>
        </div>
      </section>
    </div>
  );
}
