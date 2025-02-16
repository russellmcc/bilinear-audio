import { describe, expect, test, mock, beforeEach } from "bun:test";
import { useEnumSlider } from "./useEnumSlider";
import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";

const LINE_SPACING = 10;

const TestContainer = ({
  ref,
  children,
}: {
  ref: React.RefObject<HTMLDivElement | null>;
  children: React.ReactNode;
}) => (
  <div ref={ref} data-testid="slider-container">
    {children}
  </div>
);

const TestBall = ({
  ref,
  onPointerDown,
  onPointerMove,
  onPointerUp,
  onPointerCancel,
  bottom,
}: {
  ref: React.RefObject<HTMLDivElement | null>;
  onPointerDown: (event: React.PointerEvent) => void;
  onPointerMove: (event: React.PointerEvent) => void;
  onPointerUp: (event: React.PointerEvent) => void;
  onPointerCancel: (event: React.PointerEvent) => void;
  bottom?: number;
}) => (
  <div
    ref={ref}
    data-testid="slider-ball"
    data-bottom={bottom}
    onPointerDown={onPointerDown}
    onPointerMove={onPointerMove}
    onPointerUp={onPointerUp}
    onPointerCancel={onPointerCancel}
  />
);

const TestSlider = ({
  selectIndex,
  onGrabOrRelease,
  index,
}: {
  selectIndex?: (index: number) => void;
  onGrabOrRelease?: (grabbed: boolean) => void;
  index?: number;
}) => {
  const {
    containerRef,
    ballRef,
    onPointerDown,
    onPointerMove,
    onPointerUp,
    onPointerCancel,
    ball,
  } = useEnumSlider<HTMLDivElement, HTMLDivElement>({
    ballMargin: 0,
    lineSpacing: LINE_SPACING,
    ballSize: 10,
    index,
    count: 10,
    selectIndex: selectIndex ?? (() => {}),
    onGrabOrRelease,
  });

  return (
    <TestContainer ref={containerRef}>
      <TestBall
        ref={ballRef}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerCancel}
        bottom={ball?.bottom}
      />
    </TestContainer>
  );
};

describe("useEnumSlider", () => {
  beforeEach(() => {
    cleanup();
  });

  test("should report grabs", () => {
    const grabbedOrReleased = mock();
    const { getByTestId } = render(
      <TestSlider onGrabOrRelease={grabbedOrReleased} index={0} />,
    );

    const ball = getByTestId("slider-ball");
    fireEvent.pointerDown(ball);

    expect(grabbedOrReleased).toHaveBeenCalledWith(true);

    fireEvent.pointerUp(ball);
    expect(grabbedOrReleased).toHaveBeenCalledWith(false);
  });

  test("ball should move when index changes", async () => {
    const { getByTestId, rerender } = render(
      <TestSlider onGrabOrRelease={() => {}} index={0} />,
    );

    const ball = getByTestId("slider-ball");
    const getPos = () => parseFloat(ball.getAttribute("data-bottom") ?? "NaN");
    const originalBottom = getPos();

    rerender(<TestSlider onGrabOrRelease={() => {}} index={1} />);

    // we expect to animate into place, so check that we haven't synchronously moved
    expect(getPos()).toBe(originalBottom);

    await waitFor(() => {
      expect(originalBottom - getPos()).toBe(LINE_SPACING);
    });
  });
});
