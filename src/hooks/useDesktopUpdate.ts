import { useEffect, useRef, useState } from "react";
import { Channel, invoke } from "@tauri-apps/api/core";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, type DownloadEvent, type Update } from "@tauri-apps/plugin-updater";
import {
  deferUpdateNotification,
  loadUpdateNotificationPreference,
  saveUpdateNotificationPreference,
  shouldNotifyForUpdate,
  type UpdateNotificationPreference,
  type UpdateReminder,
} from "@/lib/updateNotifications";
import { validateRelease } from "@/lib/updateRelease";
import { errorMessage } from "@/lib/utils";

const CHECK_INTERVAL = 6 * 60 * 60_000;
const REMINDER_INTERVAL = 60_000;

export type UpdatePhase =
  | "idle"
  | "checking"
  | "available"
  | "current"
  | "downloading"
  | "installing"
  | "installed"
  | "error";

export interface UpdateProgress {
  downloaded: number;
  total?: number;
}

export function downloadPercent(progress: UpdateProgress) {
  return progress.total
    ? Math.min(100, Math.floor((progress.downloaded / progress.total) * 100))
    : null;
}

export function useDesktopUpdate() {
  const [supported, setSupported] = useState(false);
  const [phase, setPhase] = useState<UpdatePhase>("idle");
  const [open, setOpen] = useState(false);
  const [release, setRelease] = useState<Update | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [progress, setProgress] = useState<UpdateProgress>({ downloaded: 0 });
  const [canceling, setCanceling] = useState(false);
  const update = useRef<Update | null>(null);
  const download = useRef<{ canceled: boolean } | null>(null);
  const preference = useRef<UpdateNotificationPreference>({});
  const enabled = useRef(false);
  const disposed = useRef(false);
  const checking = useRef(false);
  const installing = useRef(false);
  const installed = useRef(false);
  const dialogOpen = useRef(open);
  dialogOpen.current = open;

  async function checkForUpdate(manual: boolean) {
    if (!enabled.current || checking.current || installing.current || installed.current) return;
    checking.current = true;
    if (manual) {
      setOpen(true);
      setPhase("checking");
      setError(null);
    }
    try {
      const found = await check({ timeout: 30_000, headers: { "Cache-Control": "no-cache" } });
      if (disposed.current) {
        await found?.close();
        return;
      }
      if (found) {
        try {
          validateRelease(found.rawJson);
        } catch (failure) {
          await found.close();
          throw failure;
        }
      }
      const previous = update.current;
      update.current = found;
      setRelease(found);
      setPhase(found ? "available" : "current");
      setError(null);
      if (previous) await previous.close();
      if (
        manual ||
        (found && shouldNotifyForUpdate(found.version, preference.current, Date.now()))
      ) {
        setOpen(true);
      }
    } catch (failure) {
      if (disposed.current) return;
      if (manual) {
        setPhase("error");
        setError(`Could not check for updates. ${errorMessage(failure)}`);
      } else {
        console.error("Could not check for updates", failure);
      }
    } finally {
      checking.current = false;
    }
  }

  useEffect(() => {
    disposed.current = false;
    let checkTimer: ReturnType<typeof setInterval> | undefined;
    let reminderTimer: ReturnType<typeof setInterval> | undefined;
    const remindPending = () => {
      if (
        !dialogOpen.current &&
        update.current &&
        shouldNotifyForUpdate(update.current.version, preference.current, Date.now())
      ) {
        void checkForUpdate(false);
      }
    };
    const resume = () => {
      if (document.visibilityState === "visible") remindPending();
    };

    void (async () => {
      try {
        enabled.current = await invoke<boolean>("app_update_available");
        if (disposed.current || !enabled.current) return;
        setSupported(true);
        preference.current = loadUpdateNotificationPreference();
        document.addEventListener("visibilitychange", resume);
        await checkForUpdate(false);
        if (disposed.current) return;
        checkTimer = setInterval(() => void checkForUpdate(false), CHECK_INTERVAL);
        reminderTimer = setInterval(remindPending, REMINDER_INTERVAL);
      } catch (failure) {
        if (!disposed.current) console.error("Could not initialize updates", failure);
      }
    })();

    return () => {
      disposed.current = true;
      clearInterval(checkTimer);
      clearInterval(reminderTimer);
      document.removeEventListener("visibilitychange", resume);
      if (download.current) {
        download.current.canceled = true;
        void invoke("cancel_app_update_download").catch((failure: unknown) =>
          console.error("Could not cancel the update download", failure),
        );
      }
      const current = update.current;
      update.current = null;
      if (current && (!installing.current || download.current)) {
        void current
          .close()
          .catch((failure: unknown) => console.error("Could not release the update", failure));
      }
    };
  }, []);

  function defer(reminder: UpdateReminder) {
    if (installing.current) return;
    const current = update.current;
    if (!current) {
      setOpen(false);
      return;
    }
    const next = deferUpdateNotification(current.version, reminder, Date.now());
    try {
      saveUpdateNotificationPreference(next);
    } catch (failure) {
      setError(`Could not save the reminder. ${errorMessage(failure)}`);
      return;
    }
    preference.current = next;
    setOpen(false);
  }

  async function finishSession() {
    await invoke("finish_app_update").catch((failure: unknown) => {
      console.error("Could not end the update session", failure);
      setError("Could not end the update session. Restart Prep My Windows before trying again.");
      setOpen(true);
    });
  }

  async function install() {
    const selected = update.current;
    if (!selected || installing.current || checking.current || installed.current) return;
    installing.current = true;
    const attempt = { canceled: false };
    download.current = attempt;
    setError(null);
    setCanceling(false);
    setPhase("downloading");
    setProgress({ downloaded: 0 });
    try {
      await invoke("prepare_app_update");
      if (attempt.canceled) return;
      const onEvent = new Channel<DownloadEvent>();
      onEvent.onmessage = (event) => {
        if (attempt.canceled) return;
        if (event.event === "Started") {
          setProgress({ downloaded: 0, total: event.data.contentLength });
        } else if (event.event === "Progress") {
          setProgress((value) => ({
            ...value,
            downloaded: value.downloaded + event.data.chunkLength,
          }));
        }
      };
      const completed = await invoke<boolean>("download_app_update", {
        rid: selected.rid,
        onEvent,
      });
      if (!completed || attempt.canceled) {
        attempt.canceled = true;
        return;
      }
      download.current = null;
      setPhase("installing");
      await invoke("install_app_update", { rid: selected.rid });
      installed.current = true;
      setPhase("installed");
      await relaunch();
    } catch (failure) {
      if (!attempt.canceled) {
        setPhase((value) => (value === "installed" ? "installed" : "available"));
        setError(errorMessage(failure));
        setOpen(true);
      }
    } finally {
      download.current = null;
      await finishSession();
      if (attempt.canceled) {
        setPhase("available");
        setProgress({ downloaded: 0 });
      }
      setCanceling(false);
      installing.current = false;
    }
  }

  async function cancel() {
    const attempt = download.current;
    if (!attempt || canceling) return;
    attempt.canceled = true;
    setCanceling(true);
    setError(null);
    try {
      await invoke("cancel_app_update_download");
    } catch (failure) {
      setCanceling(false);
      setError(`Could not cancel the download. Try again. ${errorMessage(failure)}`);
      setOpen(true);
    }
  }

  async function restart() {
    try {
      await relaunch();
    } catch (failure) {
      setError(errorMessage(failure));
    }
  }

  return {
    supported,
    phase,
    open,
    release,
    error,
    progress,
    canceling,
    defer,
    install,
    cancel,
    restart,
    show: () => setOpen(true),
    hide: () => setOpen(false),
    check: () => {
      if (installing.current || installed.current) {
        setOpen(true);
        return;
      }
      void checkForUpdate(true);
    },
    close: () => defer("hour"),
  };
}

export type DesktopUpdate = ReturnType<typeof useDesktopUpdate>;
