import { useCallback, useEffect, useState } from "react";

import { loadDetectors } from "@/services/detectors";
import { parseCommandError, type CommandError } from "@/types/error";

export type DetectorStatus = "loading" | "ready" | "error";

export function useDetectors() {
  const [detectors, setDetectors] = useState<string[]>([]);
  const [status, setStatus] = useState<DetectorStatus>("loading");
  const [error, setError] = useState<CommandError | null>(null);

  const reload = useCallback(async () => {
    setStatus("loading");
    setError(null);
    try {
      const ids = await loadDetectors();
      setDetectors(ids);
      setStatus("ready");
    } catch (cause) {
      setError(parseCommandError(cause));
      setStatus("error");
    }
  }, []);

  useEffect(() => {
    void reload();
  }, [reload]);

  return { detectors, status, error, reload };
}
