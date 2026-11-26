import { Download, LoaderCircle } from "lucide-react";
import { downloadPercent, type DesktopUpdate } from "@/hooks/useDesktopUpdate";

export function UpdateStatus({ updater }: { updater: DesktopUpdate }) {
  const downloading = updater.phase === "downloading";
  if (!downloading && updater.phase !== "installing" && updater.phase !== "installed") return null;

  const percent = downloadPercent(updater.progress);
  const label = updater.canceling
    ? "Canceling…"
    : downloading
      ? `Downloading update${percent === null ? "…" : ` ${percent}%`}`
      : updater.phase === "installing"
        ? "Installing update…"
        : "Update installed";

  return (
    <div className="mr-auto flex min-w-0 items-center gap-2 text-xs text-sky-300">
      <button
        type="button"
        className="flex h-7 min-w-0 items-center gap-1.5 rounded-md px-1.5 hover:bg-white/[0.06]"
        onClick={updater.show}
        aria-haspopup="dialog"
      >
        {downloading && !updater.canceling ? (
          <Download className="size-3.5 shrink-0" />
        ) : updater.phase !== "installed" ? (
          <LoaderCircle className="size-3.5 shrink-0 animate-spin" />
        ) : null}
        <span className="truncate" role="status">
          {label}
        </span>
      </button>
      {downloading && (
        <>
          <progress
            aria-label="Update download"
            className="h-1 w-12 accent-sky-500"
            max={updater.progress.total || 1}
            value={
              updater.progress.total
                ? Math.min(updater.progress.downloaded, updater.progress.total)
                : undefined
            }
          />
          <button
            type="button"
            className="h-7 rounded-md px-1.5 text-zinc-400 hover:bg-white/[0.06] hover:text-white disabled:opacity-50"
            disabled={updater.canceling}
            onClick={() => void updater.cancel()}
          >
            Cancel
          </button>
        </>
      )}
    </div>
  );
}
