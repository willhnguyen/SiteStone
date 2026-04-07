import { describe, expect, it } from "vitest";

describe("Example Utils", () => {
  it("should add two numbers", () => {
    const add = (a: number, b: number) => a + b;
    expect(add(2, 3)).toBe(5);
  });

  it("should uppercase a string", () => {
    const uppercase = (str: string) => str.toUpperCase();
    expect(uppercase("hello")).toBe("HELLO");
  });

  it("should return true for non-empty string", () => {
    const isNotEmpty = (str: string) => str.length > 0;
    expect(isNotEmpty("test")).toBe(true);
    expect(isNotEmpty("")).toBe(false);
  });
});
