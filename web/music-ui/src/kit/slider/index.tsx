import { useCallback, useMemo, useState } from "react";
import {
  LabeledNumericProps as NumericProps,
  useAccessibleNumeric,
} from "../numeric.ts";

export type SliderProps = {
  /** Currrent value of the slider */
  value: number;

  /** True if the slider is grabbed */
  grabbed?: boolean;

  /** True if the slider is hovered */
  hover?: boolean;

  /** Callback for when the value of the slider changes. */
  onValue?: (value: number) => void;

  /**
   * Callback for when the slider is grabbed or release through a pointer event.
   */
  onGrabOrRelease: (grabbed: boolean) => void;
};

export type SliderComponent = React.ComponentType<SliderProps>;

export type LabelProps = {
  /** The label of the knob */
  label: string;

  /** True if the knob is hovered */
  hover: boolean;

  /** True if the knob is grabbed */
  grabbed: boolean;

  /** The to display for the current value of the knob */
  valueLabel?: string;
};

export type LabelComponent = React.ComponentType<LabelProps>;

export type Props = {
  /**
   * A component for the slider.
   *
   * Note, you can use the `useSlider` hook to implement a nice animated slider.
   */
  Slider: SliderComponent;

  /**
   * A component for the slider label
   */
  Label?: LabelComponent;
} & NumericProps;

export const Slider = ({
  value,
  grabbed,
  onGrabOrRelease,
  onValue,
  label,
  valueFormatter,
  showLabel = "after",
  accessibilityLabel,
  defaultValue,
  Slider,
  Label,
}: Props) => {
  const valueLabel = useMemo(
    () => (valueFormatter ? valueFormatter(value) : undefined),
    [valueFormatter, value],
  );
  const { interacted, props: accessibleProps } = useAccessibleNumeric({
    value,
    onValue,
    label,
    accessibilityLabel,
    valueFormatter,
  });
  const [mouseHover, setMouseHover] = useState(false);
  const onMouseEnter = useCallback(() => {
    setMouseHover(true);
  }, []);
  const onMouseLeave = useCallback(() => {
    setMouseHover(false);
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

  const hover = interacted || mouseHover;

  const wrappedOnGrabOrRelease = useCallback(
    (grabbed: boolean) => {
      onGrabOrRelease?.(grabbed);
    },
    [onGrabOrRelease],
  );
  const labelElem =
    showLabel !== "hidden" && Label ? (
      <Label
        label={label}
        hover={hover}
        grabbed={grabbed ?? false}
        valueLabel={valueLabel}
      />
    ) : undefined;
  return (
    <div
      {...accessibleProps}
      style={{
        display: "inline-block",
        cursor: "pointer",
        touchAction: "none",
        userSelect: "none",
        WebkitUserSelect: "none",
      }}
      onDoubleClick={onDoubleClick}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
    >
      {showLabel === "before" && labelElem}
      {
        <Slider
          value={value}
          grabbed={grabbed ?? false}
          hover={hover}
          onGrabOrRelease={wrappedOnGrabOrRelease}
          onValue={onValue}
        />
      }
      {showLabel === "after" && labelElem}
    </div>
  );
};

export default Slider;
