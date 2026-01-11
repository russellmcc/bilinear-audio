import useGesture from "./useGesture.ts";
import { useMemo } from "react";
import {
  PropsWithLabel as NumericProps,
  useAccessibleNumeric,
} from "../numeric.ts";

export type DisplayProps = {
  /** Currrent value of the knob */
  value: number;

  /** True if the knob is grabbed */
  grabbed: boolean;

  /** True if the knob is hovered */
  hover: boolean;
};

export type DisplayComponent = React.ComponentType<DisplayProps>;

export type LabelProps = {
  /** The label of the knob */
  label: string;

  /** True if the knob is hovered */
  hover: boolean;

  /** True if the knob is grabbed */
  grabbed: boolean;

  /** The to display for the current value of the knob */
  valueLabel: string;
};

export type LabelComponent = React.ComponentType<LabelProps>;

export type Props = {
  /**
   * A component for the knob display
   */
  Display?: DisplayComponent;

  /**
   * A component for the knob label
   */
  Label?: LabelComponent;
} & NumericProps;

export const Knob = ({
  value,
  grabbed,
  onGrabOrRelease,
  onValue,
  label,
  valueFormatter,
  showLabel = true,
  accessibilityLabel,
  defaultValue,
  Display,
  Label,
}: Props) => {
  const { hover: mouseHover, props } = useGesture({
    value,
    onGrabOrRelease,
    onValue,
    defaultValue,
  });
  const valueLabel = useMemo(
    () => (valueFormatter ? valueFormatter(value) : label),
    [valueFormatter, value, label],
  );
  const { interacted, props: accessibleProps } = useAccessibleNumeric({
    value,
    onValue,
    label,
    accessibilityLabel,
    valueFormatter,
  });
  const hover = interacted || mouseHover;
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
      {...props}
    >
      {Display && (
        <Display value={value} grabbed={grabbed ?? false} hover={hover} />
      )}
      {showLabel && Label && (
        <Label
          label={label}
          hover={hover}
          grabbed={grabbed ?? false}
          valueLabel={valueLabel}
        />
      )}
    </div>
  );
};

export default Knob;
