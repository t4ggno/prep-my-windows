import { useState } from "react";
import { Download, LoaderCircle, RotateCcw, Upload } from "lucide-react";
import { ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { Select } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import type { AppSettings, RuntimeSettings, SystemInfo } from "@/types";

const hours = Array.from({ length: 24 }, (_, hour) => hour);
const MAX_ACTIVE_HOURS = 18;

function hourLabel(hour: number) {
  return `${hour.toString().padStart(2, "0")}:00`;
}

function activeHoursError(settings: RuntimeSettings) {
  const span = (settings.activeHoursEnd - settings.activeHoursStart + 24) % 24;
  if (span === 0) return "Start and end must differ";
  if (span > MAX_ACTIVE_HOURS) return `Active hours can span at most ${MAX_ACTIVE_HOURS} hours`;
  return null;
}

function savedSettings(settings: AppSettings): RuntimeSettings {
  return {
    startWithWindows: settings.startWithWindows,
    activeHoursStart: settings.activeHoursStart,
    activeHoursEnd: settings.activeHoursEnd,
  };
}

function HourSelect({
  value,
  onChange,
  invalid,
  label,
}: {
  value: number;
  onChange: (hour: number) => void;
  invalid: boolean;
  label: string;
}) {
  return (
    <Select
      className="w-28"
      value={value}
      aria-label={label}
      aria-invalid={invalid || undefined}
      onChange={(event) => onChange(Number(event.target.value))}
    >
      {hours.map((hour) => (
        <option key={hour} value={hour}>
          {hourLabel(hour)}
        </option>
      ))}
    </Select>
  );
}

export function SettingsView({
  settings,
  system,
  pending,
  onSave,
  onExport,
  onImport,
  onReset,
}: {
  settings: AppSettings;
  system: SystemInfo;
  pending: ReadonlySet<string>;
  onSave: (settings: RuntimeSettings) => Promise<boolean>;
  onExport: () => void;
  onImport: () => void;
  onReset: () => void;
}) {
  const saved = savedSettings(settings);
  const [draft, setDraft] = useState<RuntimeSettings | null>(null);
  const values = draft ?? saved;
  const dirty =
    draft !== null &&
    (draft.startWithWindows !== saved.startWithWindows ||
      draft.activeHoursStart !== saved.activeHoursStart ||
      draft.activeHoursEnd !== saved.activeHoursEnd);
  const validation = activeHoursError(values);
  const saving = pending.has("settings");
  const profileBusy = pending.has("profile");

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
            <HourSelect
              label="Active hours start"
              value={values.activeHoursStart}
              invalid={validation !== null}
              onChange={(activeHoursStart) => update({ activeHoursStart })}
            />
            <span>to</span>
            <HourSelect
              label="Active hours end"
              value={values.activeHoursEnd}
              invalid={validation !== null}
              onChange={(activeHoursEnd) => update({ activeHoursEnd })}
            />
          </div>
        </div>
        {validation && (
          <div className="field-error" role="alert">
            {validation}
          </div>
        )}
        <label className="settings-row">
          <span>Start with Windows</span>
          <Switch
            checked={values.startWithWindows}
            onCheckedChange={(startWithWindows) => update({ startWithWindows })}
          />
        </label>
        <div className="settings-actions">
          <Button onClick={save} disabled={!dirty || validation !== null || saving}>
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
