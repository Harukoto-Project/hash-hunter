import { HashHighlight } from "@/components/hunt/HashHighlight";
import type { RankedHash } from "@/types/scan";

type RankingItemProps = {
  rank: number;
  item: RankedHash;
};

export function RankingItem({ rank, item }: RankingItemProps) {
  return (
    <article className="grid grid-cols-[3ch_7ch_minmax(0,1fr)] gap-x-3 gap-y-1">
      <span className="text-right font-mono text-sm text-stone-500 dark:text-stone-400">
        {rank}.
      </span>
      <span className="text-right font-mono text-sm text-stone-700 dark:text-stone-300">
        {item.score.toFixed(2)}
      </span>
      <HashHighlight hash={item.hash} hits={item.hits} />
      {item.hits.map((hit) => (
        <div
          key={`${hit.detectorId}-${hit.start}-${hit.end}`}
          className="col-span-2 col-start-2 grid grid-cols-[16ch_auto_7ch] gap-x-3 font-mono text-xs"
        >
          <span className="text-stone-600 dark:text-stone-400">
            {hit.detectorId}
          </span>
          <span className="text-stone-500 dark:text-stone-400">
            {hit.start}..{hit.end}
          </span>
          <span className="text-right text-stone-700 dark:text-stone-300">
            {hit.score.toFixed(2)}
          </span>
        </div>
      ))}
    </article>
  );
}
