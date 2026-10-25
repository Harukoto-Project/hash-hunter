export type ScanInput = {
  repo: string;
  top: number;
  minScore: number;
  branch: string | null;
  only: string[];
  exclude: string[];
};

export type Hit = {
  detectorId: string;
  start: number;
  end: number;
  score: number;
};

export type RankedHash = {
  hash: string;
  score: number;
  hits: Hit[];
};

export type ScanSnapshot = {
  input: ScanInput;
  ranking: RankedHash[];
};
