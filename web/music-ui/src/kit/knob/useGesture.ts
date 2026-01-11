import { Handler, useDrag } from "@use-gesture/react";
import { useCallback, useEffect, useRef, useState } from "react";

export type GestureProps = {
  value: number;
  defaultValue?: number;
  onGrabOrRelease?: (grabbed: boolean) => void;
  onValue?: (value: number) => void;
};

const sensitivity = 1.0;
const shiftSensitivity = 0.1;

const useGesture = ({
  value,
  defaultValue,
  onGrabOrRelease,
  onValue,
}: GestureProps) => {
  const lastValue = useRef<number>(value);
  useEffect(() => {
    lastValue.current = value;
  }, [value]);

  const grabCallback: Handler<"drag"> = useCallback(
    ({ active, delta, memo, shiftKey }) => {
      if (memo === undefined) {
        memo = lastValue.current;
      }

      const last = memo as number;

      const diff = delta[1] * -(shiftKey ? shiftSensitivity : sensitivity);
      const newValue = Math.min(100, Math.max(0, last + diff));

      onValue?.(newValue);
      onGrabOrRelease?.(active);
      return newValue;
    },
    [onGrabOrRelease, onValue],
  );

  const bindDrag = useDrag(grabCallback, {
    transform: ([x, y]) => [x, y],
    pointer: {
      keys: false,
    },
  });

  const [hover, setHover] = useState(false);
  const onMouseEnter = useCallback(() => {
    setHover(true);
  }, []);
  const onMouseLeave = useCallback(() => {
    setHover(false);
  }, []);

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

  return {
    hover: hover,
    props: {
      ...bindDrag(),
      onMouseEnter,
      onMouseLeave,
      onDoubleClick,
    },
  };
};

export default useGesture;
