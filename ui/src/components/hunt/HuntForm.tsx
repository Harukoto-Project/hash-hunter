import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";

export type HuntFormValues = {
  repo: string;
  top: number;
  minScore: number;
  branch: string;
  enabledIds: string[];
};

type HuntFormProps = {
  values: HuntFormValues;
  detectorIds: string[];
  detectorsLoading: boolean;
  onChange: (values: HuntFormValues) => void;
};

export function HuntForm({
  values,
  detectorIds,
  detectorsLoading,
  onChange,
}: HuntFormProps) {
  return (
    <div className="mt-4 grid gap-4">
      <div className="grid gap-1.5">
        <Label htmlFor="repo">リポジトリ</Label>
        <Input
          id="repo"
          name="repo"
          value={values.repo}
          onChange={(event) =>
            onChange({ ...values, repo: event.target.value })
          }
          placeholder="URLまたはローカルパス"
          autoComplete="off"
        />
      </div>
      <div className="grid gap-4 sm:grid-cols-3">
        <div className="grid gap-1.5">
          <Label htmlFor="top">上位件数</Label>
          <Input
            id="top"
            name="top"
            type="number"
            min={1}
            value={values.top}
            onChange={(event) =>
              onChange({
                ...values,
                top: Math.max(1, Number(event.target.value) || 1),
              })
            }
          />
        </div>
        <div className="grid gap-1.5">
          <Label htmlFor="min-score">最小スコア</Label>
          <Input
            id="min-score"
            name="minScore"
            type="number"
            min={0}
            step={0.01}
            value={values.minScore}
            onChange={(event) =>
              onChange({
                ...values,
                minScore: Math.max(0, Number(event.target.value) || 0),
              })
            }
          />
        </div>
        <div className="grid gap-1.5">
          <Label htmlFor="branch">ブランチ</Label>
          <Input
            id="branch"
            name="branch"
            value={values.branch}
            onChange={(event) =>
              onChange({ ...values, branch: event.target.value })
            }
            placeholder="未指定は全ブランチ"
            autoComplete="off"
          />
        </div>
      </div>
      <div className="grid gap-1.5">
        <Label>検出器</Label>
        {detectorsLoading ? (
          <div className="flex flex-wrap gap-0">
            <Skeleton className="h-8 w-28 rounded-sm" />
            <Skeleton className="h-8 w-24 rounded-sm" />
            <Skeleton className="h-8 w-32 rounded-sm" />
          </div>
        ) : (
          <ToggleGroup
            type="multiple"
            variant="outline"
            size="sm"
            spacing={0}
            value={values.enabledIds}
            onValueChange={(enabledIds) => onChange({ ...values, enabledIds })}
            className="flex w-full flex-wrap"
          >
            {detectorIds.map((id) => (
              <ToggleGroupItem
                key={id}
                value={id}
                className="cursor-pointer font-mono text-xs"
              >
                {id}
              </ToggleGroupItem>
            ))}
          </ToggleGroup>
        )}
      </div>
    </div>
  );
}
