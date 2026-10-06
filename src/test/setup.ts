import { afterEach } from "vitest";
import { cleanup } from "@testing-library/react";

// Testing Library only cleans up automatically when Vitest globals are on.
// We keep them off, so unmount rendered components after every test by hand.
afterEach(() => {
  cleanup();
});
