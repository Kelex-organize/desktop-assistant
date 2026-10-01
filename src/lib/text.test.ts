import { describe, it, expect } from "vitest";
import { capitalize } from "./text";

describe("capitalize", () => {
  it("uppercases the first letter", () => {
    expect(capitalize("emanuel")).toBe("Emanuel");
  });

  it("upercase the empty string", () => {
    expect(capitalize("")).toBe("");
  });
});
