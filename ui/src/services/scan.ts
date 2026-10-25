import { invoke } from "@tauri-apps/api/core";

import type { ScanInput, ScanSnapshot } from "@/types/scan";

export async function runScan(input: ScanInput): Promise<ScanSnapshot> {
  return invoke("scan", { input });
}

export async function loadLastScan(): Promise<ScanSnapshot | null> {
  return invoke("load_last_scan");
}
