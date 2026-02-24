import { describe, expect, test, afterEach } from "bun:test";
import { render, screen, cleanup } from "@testing-library/react";
import { Root } from "./Root.tsx";

afterEach(cleanup);

describe("main", () => {
  test("app renders under full provider stack without throwing", () => {
    render(<Root />);
    expect(screen.getByRole("button", { name: "Next mode" })).toBeDefined();
  });
});
