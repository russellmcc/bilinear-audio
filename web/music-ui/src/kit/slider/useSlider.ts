import { RefObject, useCallback, useEffect, useRef, useState } from "react";
import { clamp } from "../../util";

export type Props = {
  value: number;
  ballMargin: number;
  ballSize: number;
  onValue?: (value: number) => void;
  onGrabOrRelease: (grabbed: boolean) => void;
};

const useWatchElementHeight = <Container extends HTMLElement>() => {
  const [height, setHeight] = useState<number | null>(null);
  const elem = useRef<Container | null>(null);
  const ref = useCallback((element: Container) => {
    elem.current = element;
    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        setHeight(entry.contentRect.height);
      }
    });
    observer.observe(element);
    return () => {
      observer.disconnect();
      setHeight(null);
      elem.current = null;
    };
  }, []);

  return {
    ref,
    elem,
    height,
  };
};

const shiftSensitivityScale = 0.1;

const calcContainerContentYBounds = (
  containerElem: RefObject<HTMLElement | null>,
) => {
  if (containerElem.current == undefined) {
    return { top: 0, bottom: 0 };
  }
  const boundingClientRect = containerElem.current.getBoundingClientRect();
  const style = window.getComputedStyle(containerElem.current);
  const topOffset =
    parseFloat(style.borderTopWidth) + parseFloat(style.paddingTop);
  const bottomOffset =
    parseFloat(style.borderBottomWidth) + parseFloat(style.paddingBottom);
  return {
    top: boundingClientRect.top + topOffset,
    bottom: boundingClientRect.bottom - bottomOffset,
  };
};

const eventToValue = ({
  event,
  ballSize,
  ballMargin,
  containerContentYBounds,
}: {
  event: React.PointerEvent;
  ballSize: number;
  ballMargin: number;
  containerContentYBounds: { top: number; bottom: number };
}) => {
  const trackHeight =
    containerContentYBounds.bottom -
    containerContentYBounds.top -
    ballSize -
    ballMargin * 2;

  const fromBottom =
    containerContentYBounds.bottom - event.clientY - ballSize / 2;
  const desired = ((fromBottom - ballMargin) / trackHeight) * 100;
  return desired;
};

type TouchState = {
  lastDesiredValue: number;
  lastValue: number;
};

// This is a false positive: we really need the parameter to control the return type.
// eslint-disable-next-line @typescript-eslint/no-unnecessary-type-parameters
export const useSlider = <Container extends HTMLElement = HTMLDivElement>({
  value,
  ballMargin,
  ballSize,
  onValue,
  onGrabOrRelease,
}: Props) => {
  const {
    ref: containerRef,
    elem: containerElem,
    height: containerHeight,
  } = useWatchElementHeight<Container>();
  const lastValue = useRef<number>(value);
  useEffect(() => {
    lastValue.current = value;
  }, [value]);

  const touches = useRef<Map<number, TouchState>>(new Map());
  const onPointerDown = useCallback(
    (event: React.PointerEvent) => {
      // happy-dom which we use for testing doesn't support setPointerCapture
      // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
      containerElem.current?.setPointerCapture?.(event.pointerId);

      const containerContentYBounds =
        calcContainerContentYBounds(containerElem);
      const desired = eventToValue({
        event,
        containerContentYBounds,
        ballSize,
        ballMargin,
      });

      const ballMarginInValues =
        (ballSize /
          (containerContentYBounds.bottom -
            containerContentYBounds.top -
            ballSize -
            ballMargin * 2) /
          2) *
        100;
      const clamped = clamp(desired, 0, 100);
      if (Math.abs(desired - lastValue.current) < ballMarginInValues) {
        // Special case: if the click was _on_ the ball, we don't want to jump to the desired value.
        touches.current.set(event.pointerId, {
          lastDesiredValue: desired,
          lastValue: lastValue.current,
        });
      } else {
        touches.current.set(event.pointerId, {
          lastDesiredValue: desired,
          lastValue: clamped,
        });
        onValue?.(clamped);
      }
      if (touches.current.size === 1) {
        onGrabOrRelease(true);
      }
    },
    [onGrabOrRelease, onValue, ballMargin, ballSize, containerElem],
  );
  const onPointerMove = useCallback(
    (event: React.PointerEvent) => {
      const touchState = touches.current.get(event.pointerId);
      if (!touchState) {
        return;
      }
      const containerContentYBounds =
        calcContainerContentYBounds(containerElem);
      const desired = eventToValue({
        event,
        containerContentYBounds,
        ballSize,
        ballMargin,
      });
      const delta = desired - touchState.lastDesiredValue;
      const effectiveDelta =
        delta * (event.shiftKey ? shiftSensitivityScale : 1);
      const clamped = clamp(effectiveDelta + touchState.lastValue, 0, 100);
      onValue?.(clamped);
      touches.current.set(event.pointerId, {
        lastDesiredValue: desired,
        lastValue: clamped,
      });
    },
    [onValue, ballMargin, ballSize, containerElem],
  );
  const onPointerUp = useCallback(
    (event: React.PointerEvent) => {
      touches.current.delete(event.pointerId);
      if (touches.current.size === 0) {
        onGrabOrRelease(false);
      }
    },
    [onGrabOrRelease],
  );
  const onPointerCancel = onPointerUp;

  const trackHeight =
    containerHeight != undefined
      ? containerHeight - ballSize - ballMargin * 2
      : 0;
  return {
    ballBottom: (value / 100) * trackHeight + ballMargin,
    containerProps: {
      onPointerDown,
      onPointerMove,
      onPointerUp,
      onPointerCancel,
      ref: containerRef,
    },
  };
};

export default useSlider;
