import { useCallback, useState } from "react";
import { clamp } from "music-ui/util";
import { PropsWithLabel } from "./with-label";

const KEYBOARD_STEP = 10;
const BIG_KEYBOARD_STEP = 25;

/**
 * Props shared for all numeric controls
 */
export type Props = {
  /**
   * The current value of the control (scaled to 0-100)
   */
  value: number;

  /**
   * True if the control is grabbed
   */
  grabbed?: boolean;

  /**
   * Callback for when the value of the control changes.
   * Note that this may be called spuriously even if the value didn't change.
   */
  onValue?: (value: number) => void;

  /**
   * Callback for when the control is grabbed or release through a pointer event.
   * Note that this may be called spruriously even if the grabbed state didn't change.
   */
  onGrabOrRelease?: (grabbed: boolean) => void;

  /**
   * Label for accessibility
   */
  accessibilityLabel: string;

  /**
   * Value to reset the control to on reset-to-default gesture
   */
  defaultValue?: number;

  /**
   * Value formatter to convert values into strings
   */
  valueFormatter?: (value: number) => string;
};

export type LabeledNumericProps = PropsWithLabel<Props>;

export type RequiredPropsForAria = Pick<
  PropsWithLabel<Props>,
  "label" | "accessibilityLabel" | "value" | "valueFormatter" | "onValue"
>;

/**
 * Sets up aria props, and keyboard handling for numerical controls (knobs, sliders, etc.)
 */
export const useAccessibleNumeric = ({
  value,
  onValue,
  label,
  accessibilityLabel,
  valueFormatter,
}: RequiredPropsForAria) => {
  const [interacted, setInteracted] = useState(false);

  const onBlur = useCallback(() => {
    setInteracted(false);
  }, []);

  const onKeyDown: React.KeyboardEventHandler = useCallback(
    (event) => {
      const setValue = (v: number) => {
        onValue?.(v);
        setInteracted(true);
        event.preventDefault();
        event.stopPropagation();
      };
      switch (event.code) {
        case "ArrowUp":
        case "ArrowRight":
          setValue(clamp(value + KEYBOARD_STEP, 0, 100));
          break;
        case "ArrowDown":
        case "ArrowLeft":
          setValue(clamp(value - KEYBOARD_STEP, 0, 100));
          break;
        case "PageUp":
          setValue(clamp(value + BIG_KEYBOARD_STEP, 0, 100));
          break;
        case "PageDown":
          setValue(clamp(value - BIG_KEYBOARD_STEP, 0, 100));
          break;
        case "End":
          setValue(100);
          break;
        case "Home":
          setValue(0);
          break;
      }
    },
    [onValue, value],
  );

  return {
    props: {
      role: "slider",
      onKeyDown,
      onBlur,
      "aria-label": accessibilityLabel ?? label,
      "aria-valuemin": 0,
      "aria-valuemax": 100,
      "aria-valuenow": value,
      "aria-orientation": "vertical",
      "aria-valuetext": valueFormatter ? valueFormatter(value) : String(value),
      tabIndex: 0,
    },
    interacted,
  } as const;
};
