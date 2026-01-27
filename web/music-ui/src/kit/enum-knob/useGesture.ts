import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useSmoothedValue } from "../../animation/useSmoothedValue";
import { clamp } from "../../util";

export type UseGestureProps = {
  value?: string;
  values: string[];
  onValue?: (value: string) => void;
  onGrabOrRelease?: (grabbed: boolean) => void;
  defaultValue?: string;
};

type TouchState = {
  lastTouchPos: number;
  lastTempValue: number;
};

const SCALE = 100;
const MIN_PIXELS_PER_VALUE = 3;
const SCALE_SHIFT = 0.1;

const scaleDelta = (delta: number, valueCount: number, shiftKey: boolean) => {
  const pixelsPerValue = Math.max(
    MIN_PIXELS_PER_VALUE,
    Math.floor(SCALE / valueCount),
  );
  return (delta * -(shiftKey ? SCALE_SHIFT : 1)) / pixelsPerValue;
};

export const useGesture = ({
  defaultValue,
  value,
  values,
  onValue,
  onGrabOrRelease,
}: UseGestureProps) => {
  const [hover, setHover] = useState(false);
  const wrappedOnGrabOrRelease = useCallback(
    (grabbed: boolean) => {
      onGrabOrRelease?.(grabbed);
    },
    [onGrabOrRelease],
  );
  const valueNumber = useMemo(
    () => (value ? values.indexOf(value) : undefined),
    [value, values],
  );
  const lastValue = useRef<number>(valueNumber ?? 0);
  useEffect(() => {
    lastValue.current = valueNumber ?? 0;
  }, [valueNumber]);
  const [tempValue, setTempValue] = useState<number | undefined>(undefined);
  const displayValue = useSmoothedValue(tempValue ?? valueNumber ?? 0, {
    time: tempValue !== undefined ? 0.01 : undefined,
  });

  const touches = useRef<Map<number, TouchState>>(new Map());
  const containerElem = useRef<HTMLDivElement>(null);

  const onDoubleClick: React.MouseEventHandler = useCallback(
    (event) => {
      if (defaultValue !== undefined) {
        event.preventDefault();
        event.stopPropagation();
        onValue?.(defaultValue);
      }
    },
    [defaultValue, onValue],
  );
  const onPointerDown: React.PointerEventHandler = useCallback(
    (event) => {
      containerElem.current?.setPointerCapture(event.pointerId);
      touches.current.set(event.pointerId, {
        lastTouchPos: event.clientY,
        lastTempValue: lastValue.current,
      });
      if (touches.current.size === 1) {
        wrappedOnGrabOrRelease(true);
      }
    },
    [wrappedOnGrabOrRelease],
  );
  const onPointerMove = useCallback(
    (event: React.PointerEvent) => {
      const touchState = touches.current.get(event.pointerId);
      if (!touchState) {
        return;
      }
      const delta = event.clientY - touchState.lastTouchPos;
      const oldTempValue = touchState.lastTempValue;
      const scaledDelta = scaleDelta(delta, values.length, event.shiftKey);
      const newTempValue = clamp(
        oldTempValue + scaledDelta,
        0,
        values.length - 1,
      );
      if (Math.round(newTempValue) !== Math.round(oldTempValue)) {
        onValue?.(values[Math.round(newTempValue)]!);
      }
      const springDistance = (newTempValue - Math.round(newTempValue)) * 2;
      const x =
        Math.pow(Math.abs(springDistance), 2) * (springDistance > 0 ? 1 : -1);
      setTempValue(Math.round(newTempValue) + x / 2);
      touches.current.set(event.pointerId, {
        lastTouchPos: event.clientY,
        lastTempValue: newTempValue,
      });
    },
    [values, onValue],
  );
  const onPointerUp = useCallback(
    (event: React.PointerEvent) => {
      touches.current.delete(event.pointerId);
      if (touches.current.size === 0) {
        setTempValue(undefined);
        wrappedOnGrabOrRelease(false);
      }
    },
    [wrappedOnGrabOrRelease],
  );

  const onMouseEnter = useCallback(() => {
    setHover(true);
  }, []);
  const onMouseLeave = useCallback(() => {
    setHover(false);
  }, []);
  return {
    props: {
      onDoubleClick,
      onMouseEnter,
      onMouseLeave,
      onPointerDown,
      onPointerMove,
      onPointerUp,
      ref: containerElem,
    },
    hover,
    displayValue,
  };
};

export default useGesture;
