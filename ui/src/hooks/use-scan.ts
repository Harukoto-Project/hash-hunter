import { useCallback, useEffect, useState } from "react";

import { loadLastScan, runScan } from "@/services/scan";
import { parseCommandError, type CommandError } from "@/types/error";
import type { ScanInput, ScanSnapshot } from "@/types/scan";

export type ScanStatus = "loading" | "ready" | "scanning" | "error";

export function useScan() {
  const [snapshot, setSnapshot] = useState<ScanSnapshot | null>(null);
  const [status, setStatus] = useState<ScanStatus>("loading");
  const [error, setError] = useState<CommandError | null>(null);

  useEffect(() => {
    let cancelled = false;
    loadLastScan()
      .then((last) => {
        if (cancelled) {
          return;
        }
        setSnapshot(last);
        setStatus("ready");
      })
      .catch(() => {
        if (cancelled) {
          return;
        }
        setStatus("ready");
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const scan = useCallback(async (input: ScanInput) => {
    setStatus("scanning");
    setError(null);
    try {
      const result = await runScan(input);
      setSnapshot(result);
      setStatus("ready");
    } catch (cause) {
      setError(parseCommandError(cause));
      setStatus("error");
    }
  }, []);

  return { snapshot, status, error, scan };
}
