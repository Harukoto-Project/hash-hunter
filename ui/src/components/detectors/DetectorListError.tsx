import { Button } from "@/components/ui/button";
import type { CommandError } from "@/types/error";

type DetectorListErrorProps = {
  error: CommandError;
  onRetry: () => void;
};

export function DetectorListError({ error, onRetry }: DetectorListErrorProps) {
  return (
    <div className="space-y-3">
      <p className="font-mono text-sm text-red-700 dark:text-red-400">
        error[{error.code}]: {error.message}
      </p>
      <Button
        type="button"
        variant="outline"
        className="cursor-pointer"
        onClick={onRetry}
      >
        再試行
      </Button>
    </div>
  );
}
