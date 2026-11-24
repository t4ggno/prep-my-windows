import { useCallback, useRef, useState } from "react";

export function useResource<T>(load: () => Promise<T[]>, onError: (error: unknown) => void) {
  const [items, setItems] = useState<T[]>([]);
  const [loading, setLoading] = useState(false);
  const [attempted, setAttempted] = useState(false);
  const request = useRef(0);

  const reload = useCallback(async () => {
    const current = ++request.current;
    setLoading(true);
    try {
      const next = await load();
      if (current !== request.current) return;
      setItems(next);
    } catch (caught) {
      if (current === request.current) onError(caught);
    } finally {
      if (current === request.current) {
        setLoading(false);
        setAttempted(true);
      }
    }
  }, [load, onError]);

  return { items, loading, attempted, reload };
}
