import { describe, expect, test, mock, beforeEach } from "bun:test";
import { useSlider } from "./useSlider";
import Slider, { Props, SliderProps } from ".";
import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { useCallback, useState } from "react";

const BALL_SIZE = 10;
const BALL_MARGIN = 1;
const BORDER_WIDTH = 1;
const HEIGHT = 150;

const TestDisplaySlider = ({
  value,
  onValue,
  grabbed,
  onGrabOrRelease,
}: SliderProps) => {
  const { containerProps, ballBottom } = useSlider({
    ballMargin: BALL_MARGIN,
    ballSize: BALL_SIZE,
    onGrabOrRelease,
    value,
    onValue,
  });
  return (
    <div
      style={{
        height: `${HEIGHT}px`,
        width: `${BALL_SIZE + BALL_MARGIN * 2}px`,
        borderWidth: `${BORDER_WIDTH}px`,
        position: "relative",
        borderStyle: "solid",
        borderRadius: `${BALL_SIZE / 2}px`,
        marginRight: "6px",
        marginTop: `2px`,
        cursor: "pointer",
      }}
      {...containerProps}
    >
      <div>
        <div
          className={`slider-ball ${grabbed ? "slider-ball-grabbed" : ""}`}
          data-testid="slider-ball"
          style={{
            width: `${BALL_SIZE - BORDER_WIDTH * 2}px`,
            height: `${BALL_SIZE - BORDER_WIDTH * 2}px`,
            bottom: `${ballBottom}px`,
            left: `${BALL_MARGIN}px`,
            position: "absolute",
            borderRadius: `1000px`,
            borderWidth: `${BORDER_WIDTH}px`,
            borderStyle: "solid",
          }}
        ></div>
      </div>
    </div>
  );
};

const TestSlider = ({
  grabbed,
  onGrabOrRelease,
}: Omit<Props, "Slider" | "label" | "value" | "onValue">) => {
  const [value, setValue] = useState(50);
  const onValue = useCallback((value: number) => {
    setValue(value);
  }, []);
  return (
    <Slider
      Slider={TestDisplaySlider}
      label="test"
      value={value}
      onValue={onValue}
      grabbed={grabbed}
      onGrabOrRelease={onGrabOrRelease}
    />
  );
};

describe("useSlider", () => {
  beforeEach(() => {
    cleanup();
  });

  test("should report grabs", () => {
    const grabbedOrReleased = mock();
    const { getByTestId } = render(
      <TestSlider onGrabOrRelease={grabbedOrReleased} />,
    );

    const ball = getByTestId("slider-ball");
    fireEvent.pointerDown(ball);
    expect(grabbedOrReleased).toHaveBeenCalledWith(true);
    fireEvent.pointerUp(ball);
    expect(grabbedOrReleased).toHaveBeenCalledWith(false);
  });

  test("responds to keyboard", async () => {
    const { getByRole, rerender } = render(<TestSlider />);

    const slider = getByRole("slider");
    const originalValue = parseFloat(
      slider.getAttribute("aria-valuenow") ?? "0",
    );
    fireEvent.pointerDown(slider);
    fireEvent.pointerUp(slider);
    fireEvent.keyDown(slider, { code: "ArrowRight" });
    rerender(<TestSlider />);
    await waitFor(() => {
      expect(
        parseFloat(slider.getAttribute("aria-valuenow") ?? "0") - originalValue,
      ).toBeGreaterThan(1);
    });
  });
});
