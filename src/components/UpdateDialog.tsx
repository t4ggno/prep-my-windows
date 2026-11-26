import { useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ArrowUpRight, Download, LoaderCircle } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogTitle } from "@/components/ui/dialog";
import { Select } from "@/components/ui/input";
import { downloadPercent, type DesktopUpdate, type UpdatePhase } from "@/hooks/useDesktopUpdate";
import type { UpdateReminder } from "@/lib/updateNotifications";
import { releaseNotesUrl } from "@/lib/updateRelease";

const titles: Partial<Record<UpdatePhase, string>> = {
  current: "You're up to date",
  checking: "Checking for updates",
  installed: "Update installed",
};

export function UpdateDialog({ updater }: { updater: DesktopUpdate }) {
  const [reminder, setReminder] = useState<UpdateReminder>("day");
  const downloading = updater.phase === "downloading";
  const working = downloading || updater.phase === "installing" || updater.phase === "checking";
  const available = updater.release !== null && updater.phase === "available";
  const percent = downloadPercent(updater.progress);

  return (
    <Dialog
      open={updater.open}
      onOpenChange={(open) => {
        if (open) return;
        if (downloading) updater.hide();
        else if (!working) updater.close();
      }}
    >
      <DialogContent
        className="w-[min(440px,calc(100vw-32px))]"
        closable={!working || downloading}
        onInteractOutside={(event) => event.preventDefault()}
        aria-describedby={updater.release ? "update-description" : undefined}
      >
        <div className="grid gap-5 p-6">
          <div className="grid gap-2 pr-6">
            <div className="mb-1 grid size-11 place-items-center rounded-xl bg-sky-500/10 text-sky-300">
              <Download className="size-5" />
            </div>
            <DialogTitle>{titles[updater.phase] ?? "Prep My Windows update"}</DialogTitle>
            {updater.release && (
              <DialogDescription id="update-description">
                {`${updater.release.currentVersion} → ${updater.release.version}`}
              </DialogDescription>
            )}
          </div>

          {available && (
            <button
              className="flex w-fit items-center gap-1 text-sm text-sky-300 hover:text-sky-200"
              onClick={() => {
                void openUrl(releaseNotesUrl(updater.release!.version)).catch((failure: unknown) =>
                  console.error("Could not open release notes", failure),
                );
              }}
            >
              Release notes <ArrowUpRight className="size-4" />
            </button>
          )}

          {downloading && (
            <div className="grid gap-2" role="status">
              <div className="flex justify-between text-sm text-zinc-300">
                <span>{updater.canceling ? "Canceling…" : "Downloading"}</span>
                {percent !== null && <span>{percent}%</span>}
              </div>
              <progress
                aria-label="Update download"
                className="h-2 w-full accent-sky-500"
                max={updater.progress.total ?? 1}
                value={updater.progress.total ? updater.progress.downloaded : undefined}
              />
            </div>
          )}
          {updater.phase === "installing" && (
            <p className="flex items-center gap-2 text-sm text-zinc-300" role="status">
              <LoaderCircle className="size-4 animate-spin" />
              Installing…
            </p>
          )}

          {updater.error && (
            <p className="text-sm text-red-300" role="alert">
              {updater.error}
            </p>
          )}

          {available && (
            <label className="grid gap-2 text-sm text-zinc-300">
              Remind me
              <Select
                value={reminder}
                onChange={(event) => setReminder(event.target.value as UpdateReminder)}
              >
                <option value="hour">In 1 hour</option>
                <option value="day">Tomorrow</option>
                <option value="week">In 1 week</option>
                <option value="release">Until the next release</option>
              </Select>
            </label>
          )}

          <div className="flex justify-end gap-2">
            {downloading ? (
              <>
                <Button
                  variant="secondary"
                  disabled={updater.canceling}
                  onClick={() => void updater.cancel()}
                >
                  {updater.canceling ? "Canceling…" : "Cancel download"}
                </Button>
                <Button variant="secondary" onClick={updater.hide}>
                  Hide
                </Button>
              </>
            ) : updater.phase === "installed" ? (
              <Button onClick={() => void updater.restart()}>Restart Prep My Windows</Button>
            ) : available ? (
              <>
                <Button variant="secondary" onClick={() => updater.defer(reminder)}>
                  {reminder === "release" ? "Skip this release" : "Remind me later"}
                </Button>
                <Button autoFocus onClick={() => void updater.install()}>
                  Update and restart
                </Button>
              </>
            ) : !working ? (
              <>
                <Button variant="secondary" onClick={updater.close}>
                  Close
                </Button>
                {updater.phase === "error" && <Button onClick={updater.check}>Retry</Button>}
              </>
            ) : null}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
