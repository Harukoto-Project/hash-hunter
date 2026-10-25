import { detectorDescription } from "@/config/detectorDescriptions";

type DetectorListProps = {
  ids: string[];
};

export function DetectorList({ ids }: DetectorListProps) {
  return (
    <div className="space-y-6">
      {ids.map((id) => (
        <div key={id} className="space-y-1">
          <p className="font-mono text-sm text-stone-700 dark:text-stone-300">
            {id}
          </p>
          <p className="text-xs text-stone-600 dark:text-stone-400">
            {detectorDescription(id)}
          </p>
        </div>
      ))}
    </div>
  );
}
