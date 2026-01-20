import { fireEvent, getByTestId, render } from "@testing-library/react";
import { describe, expect, mock, test } from "bun:test";
import EnumKnob, { DisplayProps, Props } from ".";

const TestDisplay = ({ hover }: DisplayProps) => (
  <div className={hover ? "hover" : ""} data-testid="display"></div>
);
const TestEnumKnob = (props: Omit<Props, "Display">) => (
  <EnumKnob {...props} Display={TestDisplay} />
);

describe("EnumKnob", () => {
  test("has correct value", () => {
    const { getByRole } = render(
      <TestEnumKnob label="test" value="a" values={["a", "b", "c"]} />,
    );
    const knobElement = getByRole("spinbutton");
    expect(knobElement).toBeDefined();
    expect(knobElement.getAttribute("aria-label")).toBe("test");
    expect(knobElement.getAttribute("aria-valuetext")).toBe("a");
  });
  test("can change value with keyboard", () => {
    const onValue = mock();
    const { getByRole, rerender } = render(
      <TestEnumKnob
        label="test"
        value="a"
        values={["a", "b", "c"]}
        onValue={onValue}
      />,
    );
    const knobElement = getByRole("spinbutton");
    expect(knobElement).toBeDefined();
    fireEvent.keyDown(knobElement, { code: "ArrowRight" });
    expect(onValue).toHaveBeenCalledWith("b");
    onValue.mockClear();
    fireEvent.keyDown(knobElement, { code: "ArrowUp" });
    expect(onValue).toHaveBeenCalledWith("b");
    onValue.mockClear();
    fireEvent.keyDown(knobElement, { code: "ArrowDown" });
    expect(onValue).toHaveBeenCalledWith("c");
    onValue.mockClear();
    fireEvent.keyDown(knobElement, { code: "ArrowLeft" });
    expect(onValue).toHaveBeenCalledWith("c");
    onValue.mockClear();
    fireEvent.keyDown(knobElement, { code: "End" });
    expect(onValue).toHaveBeenCalledWith("c");
    onValue.mockClear();
    rerender(
      <TestEnumKnob
        label="test"
        value="c"
        values={["a", "b", "c"]}
        onValue={onValue}
      />,
    );
    fireEvent.keyDown(knobElement, { code: "Home" });
    expect(onValue).toHaveBeenCalledWith("a");
  });
  test("can double click to reset value", () => {
    const onValue = mock();
    const { getByRole } = render(
      <TestEnumKnob
        label="test"
        value="a"
        values={["a", "b", "c"]}
        onValue={onValue}
        defaultValue="b"
      />,
    );
    const knobElement = getByRole("spinbutton");
    fireEvent.doubleClick(knobElement);
    expect(onValue).toHaveBeenCalledWith("b");
  });
  test("hover is activated by keyboard", () => {
    const { getByRole } = render(
      <TestEnumKnob label="test" value="a" values={["a", "b", "c"]} />,
    );
    const knobElement = getByRole("spinbutton");
    const display = getByTestId(knobElement, "display");
    expect(display.className).not.toContain("hover");
    fireEvent.keyDown(knobElement, { code: "ArrowRight" });
    expect(display.className).toContain("hover");
  });
});
