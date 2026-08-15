import { useEffect, useState } from "react";
import { Download, RotateCcw, Upload } from "lucide-react";
import { ViewHeader } from "@/components/common";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import type { AppSettings, RuntimeSettings, SystemInfo } from "@/types";

export function SettingsView({
  settings,
  system,
  onSave,
  onExport,
  onImport,
  onReset,
}: {
  settings: AppSettings;
  system: SystemInfo;
  onSave: (settings: RuntimeSettings) => void;
  onExport: () => void;
  onImport: () => void;
  onReset: () => void;
}) {
  const [values, setValues] = useState<RuntimeSettings>({
    startWithWindows: settings.startWithWindows,
    activeHoursStart: settings.activeHoursStart,
    activeHoursEnd: settings.activeHoursEnd,
  });

  useEffect(() => {
    setValues({
      startWithWindows: settings.startWithWindows,
      activeHoursStart: settings.activeHoursStart,
      activeHoursEnd: settings.activeHoursEnd,
    });
  }, [settings]);

  return (
    <div className="view-stack settings-view">
      <ViewHeader title="Settings" />
      <section className="settings-card">
        <div className="section-label">Enforcement</div>
        <label className="settings-row">
          <span>Active hours start</span>
          <div className="number-input">
            <Input
              type="number"
              min={0}
              max={23}
              value={values.activeHoursStart}
              onChange={(event) =>
                setValues((current) => ({
                  ...current,
                  activeHoursStart: Number(event.target.value),
                }))
              }
            />
            <span>:00</span>
          </div>
        </label>
        <label className="settings-row">
          <span>Active hours end</span>
          <div className="number-input">
            <Input
              type="number"
              min={0}
              max={23}
              value={values.activeHoursEnd}
              onChange={(event) =>
                setValues((current) => ({
                  ...current,
                  activeHoursEnd: Number(event.target.value),
                }))
              }
            />
            <span>:00</span>
          </div>
        </label>
        <div className="settings-row">
          <span>Start with Windows</span>
          <Switch
            checked={values.startWithWindows}
            onCheckedChange={(startWithWindows) =>
              setValues((current) => ({ ...current, startWithWindows }))
            }
          />
        </div>
        <div className="settings-actions">
          <Button onClick={() => onSave(values)}>Save settings</Button>
        </div>
      </section>

      <section className="settings-card">
        <div className="section-label">Profile</div>
        <div className="button-row">
          <Button variant="secondary" onClick={onExport}>
            <Download className="size-4" /> Export
          </Button>
          <Button variant="secondary" onClick={onImport}>
            <Upload className="size-4" /> Import
          </Button>
          <Button variant="secondary" onClick={onReset}>
            <RotateCcw className="size-4" /> Reset profile
          </Button>
        </div>
      </section>

      <section className="settings-card">
        <div className="section-label">System</div>
        <div className="system-grid">
          <span>Edition</span><strong>{system.productName}</strong>
          <span>Version</span><strong>{system.displayVersion}</strong>
          <span>Build</span><strong>{system.buildNumber}</strong>
          <span>Access</span><strong>{system.isElevated ? "Administrator" : "Standard user"}</strong>
        </div>
      </section>
    </div>
  );
}
