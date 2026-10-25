import type { HuntFormValues } from "@/components/hunt/HuntForm";
import type { ScanInput } from "@/types/scan";

export const DEFAULT_HUNT_FORM: HuntFormValues = {
  repo: "",
  top: 20,
  minScore: 0,
  branch: "",
  enabledIds: [],
};

export function inputToForm(
  input: ScanInput,
  detectorIds: string[],
): HuntFormValues {
  let enabledIds = detectorIds;
  if (input.only.length > 0) {
    enabledIds = input.only;
  } else if (input.exclude.length > 0) {
    enabledIds = detectorIds.filter((id) => !input.exclude.includes(id));
  }
  return {
    repo: input.repo,
    top: input.top,
    minScore: input.minScore,
    branch: input.branch ?? "",
    enabledIds,
  };
}

export function formToInput(
  values: HuntFormValues,
  detectorIds: string[],
): ScanInput {
  const allOn =
    detectorIds.length > 0 &&
    values.enabledIds.length === detectorIds.length &&
    detectorIds.every((id) => values.enabledIds.includes(id));
  return {
    repo: values.repo.trim(),
    top: values.top,
    minScore: values.minScore,
    branch: values.branch.trim() === "" ? null : values.branch.trim(),
    only: allOn ? [] : values.enabledIds,
    exclude: [],
  };
}
