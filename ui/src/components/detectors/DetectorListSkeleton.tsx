import { Skeleton } from "@/components/ui/skeleton";

export function DetectorListSkeleton() {
  return (
    <div className="space-y-6">
      {Array.from({ length: 8 }, (_, index) => (
        <div key={index} className="space-y-1">
          <Skeleton className="h-5 w-36 rounded-sm" />
          <Skeleton className="h-4 w-64 rounded-sm" />
        </div>
      ))}
    </div>
  );
}
