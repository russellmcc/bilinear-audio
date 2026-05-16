import { describe, expect, test, afterEach } from "bun:test";
import {
  render,
  screen,
  cleanup,
  fireEvent,
  waitFor,
} from "@testing-library/react";
import { Root } from "./Root.tsx";

afterEach(cleanup);

describe("main", () => {
  test("app renders under full provider stack without throwing", () => {
    render(<Root />);
    expect(screen.getByRole("button", { name: "Next mode" })).toBeDefined();
  });

  test("mode carousel reaches Jazz 120", async () => {
    render(<Root />);
    fireEvent.click(screen.getByRole("button", { name: "Next mode" }));
    await waitFor(() => {
      expect(screen.getByText(/superdimensional/)).toBeDefined();
    });
    fireEvent.click(screen.getByRole("button", { name: "Next mode" }));
    await waitFor(() => {
      expect(screen.getByText("C–2")).toBeDefined();
    });
    fireEvent.click(screen.getByRole("button", { name: "Next mode" }));
    await waitFor(() => {
      expect(
        screen.getByRole("img", { name: "FUNK CHORUS-120" }),
      ).toBeDefined();
    });
  });
});
