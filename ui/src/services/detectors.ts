import { invoke } from "@tauri-apps/api/core";

export async function loadDetectors(): Promise<string[]> {
  return invoke("list_detectors");
}
