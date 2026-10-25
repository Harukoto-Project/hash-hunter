type HuntEmptyProps = {
  scanned: boolean;
};

export function HuntEmpty({ scanned }: HuntEmptyProps) {
  return (
    <p className="text-sm text-stone-600 dark:text-stone-400">
      {scanned
        ? "該当なし。条件を変えるか別リポジトリをスキャン"
        : "リポジトリを指定してスキャン"}
    </p>
  );
}
