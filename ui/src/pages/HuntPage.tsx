import { useEffect, useRef, useState, type FormEvent } from "react";

import { HuntEmpty } from "@/components/hunt/HuntEmpty";
import { HuntError } from "@/components/hunt/HuntError";
import { HuntForm } from "@/components/hunt/HuntForm";
import { RankingList } from "@/components/hunt/RankingList";
import { RankingSkeleton } from "@/components/hunt/RankingSkeleton";
import { PageHeader } from "@/components/layout/PageHeader";
import { Button } from "@/components/ui/button";
import { useDetectors } from "@/hooks/use-detectors";
import { useScan } from "@/hooks/use-scan";
import { DEFAULT_HUNT_FORM, formToInput, inputToForm } from "@/scanForm";

export function HuntPage() {
  const { snapshot, status, error, scan } = useScan();
  const { detectors, status: detectorStatus } = useDetectors();
  const [values, setValues] = useState(DEFAULT_HUNT_FORM);
  const applied = useRef(false);

  useEffect(() => {
    if (status === "loading") {
      return;
    }
    if (applied.current) {
      return;
    }
    applied.current = true;
    if (snapshot) {
      setValues(inputToForm(snapshot.input, detectors));
      return;
    }
    if (detectors.length > 0) {
      setValues((current) => ({ ...current, enabledIds: detectors }));
    }
  }, [detectors, snapshot, status]);

  useEffect(() => {
    if (values.enabledIds.length > 0 || detectors.length === 0) {
      return;
    }
    if (
      snapshot &&
      (snapshot.input.only.length > 0 || snapshot.input.exclude.length > 0)
    ) {
      return;
    }
    setValues((current) => ({ ...current, enabledIds: detectors }));
  }, [detectors, snapshot, values.enabledIds.length]);

  const input = formToInput(values, detectors);
  const canScan =
    input.repo.length > 0 &&
    values.enabledIds.length > 0 &&
    status !== "scanning";

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!canScan) {
      return;
    }
    void scan(input);
  };

  const scanned = snapshot !== null;
  const showSkeleton = status === "loading" || status === "scanning";

  return (
    <form onSubmit={handleSubmit} className="flex min-h-0 flex-1 flex-col">
      <PageHeader
        title="探す"
        actions={
          <Button type="submit" className="cursor-pointer" disabled={!canScan}>
            スキャン
          </Button>
        }
      >
        <HuntForm
          values={values}
          detectorIds={detectors}
          detectorsLoading={detectorStatus === "loading"}
          onChange={setValues}
        />
      </PageHeader>
      <div className="min-h-0 flex-1 overflow-auto px-4 py-4 md:px-6">
        {showSkeleton ? <RankingSkeleton /> : null}
        {status === "error" && error ? (
          <HuntError error={error} onRetry={() => void scan(input)} />
        ) : null}
        {status === "ready" && scanned && snapshot.ranking.length === 0 ? (
          <HuntEmpty scanned />
        ) : null}
        {status === "ready" && !scanned ? <HuntEmpty scanned={false} /> : null}
        {status === "ready" && snapshot && snapshot.ranking.length > 0 ? (
          <RankingList items={snapshot.ranking} />
        ) : null}
      </div>
    </form>
  );
}
