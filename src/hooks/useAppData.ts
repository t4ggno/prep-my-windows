import { useCallback, useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { api } from "@/api";
import { errorMessage } from "@/lib/utils";
import type { LiveState, Snapshot } from "@/types";

export type Mutate = (
  key: string,
  action: () => Promise<unknown>,
  optimistic?: (current: Snapshot) => Snapshot,
) => Promise<boolean>;

export function useAppData() {
  const [snapshot, setSnapshot] = useState<Snapshot | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [pending, setPending] = useState<ReadonlySet<string>>(() => new Set());
  const pendingCount = useRef(0);
  const snapshotRequest = useRef(0);
  const snapshotStale = useRef(false);
  const lastLiveStatus = useRef<LiveState["status"] | null>(null);

  const loadSnapshot = useCallback(async (showLoading = false) => {
    const request = ++snapshotRequest.current;
    if (showLoading) setLoading(true);
    try {
      const next = await api.snapshot();
      if (request !== snapshotRequest.current) return;
      lastLiveStatus.current = next.status;
      setSnapshot(next);
    } catch (caught) {
      if (request === snapshotRequest.current) setError(errorMessage(caught));
    } finally {
      if (request === snapshotRequest.current) setLoading(false);
    }
  }, []);

  const mutate = useCallback<Mutate>(
    async (key, action, optimistic) => {
      snapshotRequest.current += 1;
      if (optimistic) setSnapshot((current) => (current ? optimistic(current) : current));
      pendingCount.current += 1;
      setPending((current) => new Set(current).add(key));
      let succeeded = true;
      try {
        await action();
        setError(null);
      } catch (caught) {
        succeeded = false;
        setError(errorMessage(caught));
      } finally {
        pendingCount.current -= 1;
        setPending((current) => {
          const next = new Set(current);
          next.delete(key);
          return next;
        });
      }
      if (pendingCount.current === 0) await loadSnapshot();
      return succeeded;
    },
    [loadSnapshot],
  );

  useEffect(() => {
    void loadSnapshot(true);
  }, [loadSnapshot]);

  useEffect(() => {
    let disposed = false;
    const unlisteners: Array<() => void> = [];

    function refreshWhenVisible() {
      if (pendingCount.current > 0) return;
      if (document.hidden) {
        snapshotStale.current = true;
        return;
      }
      snapshotStale.current = false;
      void loadSnapshot();
    }

    function handleLiveState(live: LiveState) {
      const previous = lastLiveStatus.current;
      lastLiveStatus.current = live.status;
      setSnapshot((current) =>
        current ? { ...current, status: live.status, activity: live.activity } : current,
      );
      if (
        previous &&
        ((previous.busy && !live.status.busy) ||
          previous.reappliedTotal !== live.status.reappliedTotal)
      ) {
        refreshWhenVisible();
      }
    }

    Promise.all([
      listen<LiveState>("live-state-changed", ({ payload }) => handleLiveState(payload)),
      listen("config-changed", refreshWhenVisible),
    ])
      .then((registered) => {
        if (disposed) registered.forEach((unlisten) => unlisten());
        else unlisteners.push(...registered);
      })
      .catch((caught) => {
        if (!disposed) setError(errorMessage(caught));
      });

    function handleVisibilityChange() {
      if (!document.hidden && snapshotStale.current) refreshWhenVisible();
    }
    document.addEventListener("visibilitychange", handleVisibilityChange);
    return () => {
      disposed = true;
      unlisteners.forEach((unlisten) => unlisten());
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, [loadSnapshot]);

  return { snapshot, setSnapshot, loading, error, setError, pending, loadSnapshot, mutate };
}
