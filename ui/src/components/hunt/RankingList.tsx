import { RankingItem } from "@/components/hunt/RankingItem";
import type { RankedHash } from "@/types/scan";

type RankingListProps = {
  items: RankedHash[];
};

export function RankingList({ items }: RankingListProps) {
  return (
    <div className="space-y-5">
      {items.map((item, index) => (
        <RankingItem key={item.hash} rank={index + 1} item={item} />
      ))}
    </div>
  );
}
