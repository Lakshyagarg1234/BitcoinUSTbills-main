import { vi } from "vitest";

export const set = vi.fn((_: string): Promise<void> => {
  return Promise.resolve();
});
