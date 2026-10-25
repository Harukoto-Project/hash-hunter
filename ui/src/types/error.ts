export type CommandError = {
  code: string;
  message: string;
};

export function parseCommandError(error: unknown): CommandError {
  if (typeof error === "object" && error !== null) {
    const record = error as Record<string, unknown>;
    if (typeof record.code === "string" && typeof record.message === "string") {
      return { code: record.code, message: record.message };
    }
  }
  return { code: "unknown", message: String(error) };
}
