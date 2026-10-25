import { Skeleton } from "@/components/ui/skeleton";

export function RankingSkeleton() {
  return (
    <div className="space-y-5">
      {Array.from({ length: 4 }, (_, index) => (
        <div
          key={index}
          className="grid grid-cols-[3ch_7ch_minmax(0,1fr)] gap-x-3 gap-y-1"
        >
          <Skeleton className="h-5 w-full rounded-sm" />
          <Skeleton className="h-5 w-full rounded-sm" />
          <Skeleton className="h-5 w-full max-w-80 rounded-sm" />
          <div className="col-span-2 col-start-2 grid grid-cols-[16ch_auto_7ch] gap-x-3">
            <Skeleton className="h-4 w-full rounded-sm" />
            <Skeleton className="h-4 w-12 rounded-sm" />
            <Skeleton className="h-4 w-full rounded-sm" />
          </div>
          <div className="col-span-2 col-start-2 grid grid-cols-[16ch_auto_7ch] gap-x-3">
            <Skeleton className="h-4 w-full rounded-sm" />
            <Skeleton className="h-4 w-12 rounded-sm" />
            <Skeleton className="h-4 w-full rounded-sm" />
          </div>
        </div>
      ))}
    </div>
  );
}
