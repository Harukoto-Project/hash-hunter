export const DETECTOR_DESCRIPTIONS: Record<string, string> = {
  "leading-zeros": "先頭の0の連続(4文字以上)",
  "trailing-zeros": "末尾の0の連続(4文字以上)",
  "repeat-run": "同じ文字の連続(4文字以上)",
  ascending: "16進の昇順連続(4文字以上)",
  descending: "16進の降順連続(4文字以上)",
  "digits-only": "ハッシュ全体が数字(0-9)",
  "letters-only": "ハッシュ全体が英字(a-f)",
  palindrome: "長さ6以上の回文",
  "repeat-pattern": "同じパターンの繰り返し",
  "hex-word": "dead beef cafe babe face c0de f00d",
  lucky: "777 1234567",
};

export function detectorDescription(id: string): string {
  return DETECTOR_DESCRIPTIONS[id] ?? id;
}
