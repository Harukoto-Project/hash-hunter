import type { Hit } from "@/types/scan";

type HashHighlightProps = {
  hash: string;
  hits: Hit[];
};

type Part = {
  text: string;
  hit: boolean;
};

function partsFromHits(hash: string, hits: Hit[]): Part[] {
  const marked = Array.from({ length: hash.length }, () => false);
  for (const hit of hits) {
    for (
      let index = hit.start;
      index < hit.end && index < hash.length;
      index += 1
    ) {
      marked[index] = true;
    }
  }
  const parts: Part[] = [];
  let index = 0;
  while (index < hash.length) {
    const hit = marked[index];
    const start = index;
    index += 1;
    while (index < hash.length && marked[index] === hit) {
      index += 1;
    }
    parts.push({ text: hash.slice(start, index), hit });
  }
  return parts;
}

export function HashHighlight({ hash, hits }: HashHighlightProps) {
  const parts = partsFromHits(hash, hits);
  return (
    <span className="font-mono text-sm break-all text-stone-700 dark:text-stone-300">
      {parts.map((part, index) =>
        part.hit ? (
          <span
            key={`${part.text}-${index}`}
            className="bg-amber-100 font-bold text-amber-900 dark:bg-amber-900 dark:text-amber-100"
          >
            {part.text}
          </span>
        ) : (
          <span key={`${part.text}-${index}`}>{part.text}</span>
        ),
      )}
    </span>
  );
}
