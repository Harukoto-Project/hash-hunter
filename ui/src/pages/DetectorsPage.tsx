import { DetectorList } from "@/components/detectors/DetectorList";
import { DetectorListError } from "@/components/detectors/DetectorListError";
import { DetectorListSkeleton } from "@/components/detectors/DetectorListSkeleton";
import { PageHeader } from "@/components/layout/PageHeader";
import { useDetectors } from "@/hooks/use-detectors";

export function DetectorsPage() {
  const { detectors, status, error, reload } = useDetectors();

  return (
    <div className="flex min-h-0 flex-1 flex-col">
      <PageHeader title="検出器" />
      <div className="min-h-0 flex-1 overflow-auto px-4 py-4 md:px-6">
        {status === "loading" ? <DetectorListSkeleton /> : null}
        {status === "error" && error ? (
          <DetectorListError error={error} onRetry={() => void reload()} />
        ) : null}
        {status === "ready" ? <DetectorList ids={detectors} /> : null}
      </div>
    </div>
  );
}
