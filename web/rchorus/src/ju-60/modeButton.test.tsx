import { describe, expect, test, afterEach } from "bun:test";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { useState } from "react";
import { JU_60_BUTTON_MODES, type Ju60ButtonMode } from "./constants";
import ModeButton from "./modeButton";

afterEach(cleanup);

const ModeButtonHarness = () => {
  const [buttonMode, setButtonMode] = useState<Ju60ButtonMode>("I");
  const selectModeByOffset = (mode: Ju60ButtonMode, offset: number) => {
    const currentIndex = JU_60_BUTTON_MODES.indexOf(mode);
    const nextIndex =
      (currentIndex + offset + JU_60_BUTTON_MODES.length) %
      JU_60_BUTTON_MODES.length;
    const nextMode = JU_60_BUTTON_MODES[nextIndex];
    if (nextMode === undefined) {
      return;
    }
    setButtonMode(nextMode);
    const input = document.getElementById(`ju-60-mode-${nextMode}`);
    if (input instanceof HTMLElement) {
      input.focus();
    }
  };

  return (
    <div role="radiogroup" aria-label="Synthesizer chorus mode">
      {JU_60_BUTTON_MODES.map((mode) => (
        <ModeButton
          key={mode}
          mode={mode}
          active={mode === buttonMode}
          onSelect={setButtonMode}
          onSelectByOffset={selectModeByOffset}
        />
      ))}
    </div>
  );
};

describe("ModeButton", () => {
  test("renders as an ARIA radio group", () => {
    render(<ModeButtonHarness />);

    expect(
      screen.getByRole("radiogroup", { name: "Synthesizer chorus mode" }),
    ).toBeDefined();
    expect(
      screen
        .getByRole("radio", { name: "Synthesizer chorus mode I" })
        .getAttribute("aria-checked"),
    ).toBe("true");
  });

  test("ArrowDown wraps around", () => {
    render(<ModeButtonHarness />);

    const modeIII = screen.getByRole("radio", {
      name: "Synthesizer chorus mode III",
    });
    fireEvent.click(modeIII);
    expect(modeIII.getAttribute("aria-checked")).toBe("true");

    fireEvent.keyDown(modeIII, { key: "ArrowDown" });

    expect(
      screen
        .getByRole("radio", { name: "Synthesizer chorus mode I" })
        .getAttribute("aria-checked"),
    ).toBe("true");
  });

  test("clicking gives keyboard focus", () => {
    render(<ModeButtonHarness />);

    const modeII = screen.getByRole("radio", {
      name: "Synthesizer chorus mode II",
    });
    fireEvent.pointerDown(modeII);
    fireEvent.click(modeII);

    expect(document.activeElement).toBe(modeII);
    expect(modeII.getAttribute("aria-checked")).toBe("true");
  });

  test("ArrowDown works after click focus", () => {
    render(<ModeButtonHarness />);

    const modeII = screen.getByRole("radio", {
      name: "Synthesizer chorus mode II",
    });
    fireEvent.pointerDown(modeII);
    fireEvent.click(modeII);
    const activeElement = document.activeElement;
    if (activeElement === null) {
      throw new Error("Expected a focused element");
    }

    fireEvent.keyDown(activeElement, { key: "ArrowDown" });

    expect(
      screen
        .getByRole("radio", { name: "Synthesizer chorus mode III" })
        .getAttribute("aria-checked"),
    ).toBe("true");
  });

  test("ArrowUp works after click focus", () => {
    render(<ModeButtonHarness />);

    const modeII = screen.getByRole("radio", {
      name: "Synthesizer chorus mode II",
    });
    fireEvent.pointerDown(modeII);
    fireEvent.click(modeII);
    const activeElement = document.activeElement;
    if (activeElement === null) {
      throw new Error("Expected a focused element");
    }

    fireEvent.keyDown(activeElement, { key: "ArrowUp" });

    expect(
      screen
        .getByRole("radio", { name: "Synthesizer chorus mode I" })
        .getAttribute("aria-checked"),
    ).toBe("true");
  });
});
