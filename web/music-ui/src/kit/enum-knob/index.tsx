import { useMemo } from "react";
import { Props as EnumProps } from "../enum";
import { PropsWithLabel } from "../with-label";
import useAccessible from "./useAccessible";
import useGesture from "./useGesture";

export type LabelProps = {
  /** The label of the knob */
  label: string;

  /** True if the knob is hovered */
  hover: boolean;

  /** True if the knob is grabbed */
  grabbed: boolean;

  /** The string to display for the current value of the knob, if one is selected */
  value?: string;
};

export type LabelComponent = React.ComponentType<LabelProps>;

export type DisplayProps = {
  /** The number of valid values */
  valueCount: number;

  /** Current (potentially fractional) value, or undefined if no value is selected */
  value?: number;

  /** String representation of the current value, or undefined if no value is selected */
  valueLabel?: string;

  /** True if the knob is grabbed */
  grabbed: boolean;

  /** True if the knob is hovered */
  hover: boolean;
};

export type DisplayComponent = React.ComponentType<DisplayProps>;

export type Props = PropsWithLabel<EnumProps> & {
  /**
   * Component type to use for the value label.
   *
   * Note that this must forward props to an HTML div element.
   */
  Label?: LabelComponent;

  /**
   * Component type to use for the display.
   */
  Display: DisplayComponent;
};

export const EnumKnob = ({
  value,
  values,
  onValue,
  grabbed = false,
  label,
  displayFormatter,
  Label,
  Display,
  accessibilityLabel,
  defaultValue,
  onGrabOrRelease,
  showLabel = "after",
}: Props) => {
  const valueNumber = useMemo(
    () => (value ? values.indexOf(value) : undefined),
    [value, values],
  );
  const {
    props: gestureProps,
    hover,
    displayValue,
  } = useGesture({
    value,
    values,
    onValue,
    defaultValue,
    onGrabOrRelease,
  });

  const valueCount = useMemo(() => values.length, [values]);

  const valueLabel = useMemo(
    () => (value ? (displayFormatter?.(value) ?? value) : undefined),
    [value, displayFormatter],
  );
  const { props: accessibleProps, interacted } = useAccessible({
    value,
    values,
    onValue,
    displayFormatter,
    label,
    accessibilityLabel,
  });

  const display = (
    <Display
      key="display"
      valueCount={valueCount}
      value={valueNumber !== undefined ? displayValue : undefined}
      valueLabel={valueLabel}
      grabbed={grabbed}
      hover={interacted || hover}
    />
  );

  const labelElem =
    showLabel !== "hidden" && Label ? (
      <Label
        key="label"
        label={label}
        hover={interacted || hover}
        grabbed={grabbed || interacted}
        value={valueLabel}
      />
    ) : undefined;

  const elems =
    showLabel === "before" ? [labelElem, display] : [display, labelElem];

  return (
    <div
      {...accessibleProps}
      {...gestureProps}
      style={{
        display: "inline-block",
        cursor: "pointer",
        touchAction: "none",
        userSelect: "none",
        WebkitUserSelect: "none",
      }}
    >
      {elems}
    </div>
  );
};

export default EnumKnob;
