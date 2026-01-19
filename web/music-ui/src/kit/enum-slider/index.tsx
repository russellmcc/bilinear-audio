import { useCallback, useEffect, useRef } from "react";
import { indexOf } from "../../util";
import { LabelGroup, ValueLabel } from "./value-label";
export type { ValueLabel, ValueLabelProps } from "./value-label";
export type SliderProps = {
  index: number | undefined;
  count: number;
  selectIndex: (index: number) => void;
  onGrabOrRelease?: (grabbed: boolean) => void;
  grabbed: boolean;
};

export type Slider = React.FC<SliderProps>;

export type Layout = "slider-first" | "labels-first";

export type Props = {
  /**
   * The possible values of the enum
   */
  values: string[];

  /**
   * True if the slider is grabbed
   */
  grabbed?: boolean;

  /**
   * The current value of the enum - must be one of `values`
   */
  value?: string;

  /**
   * Accessibility label for the enum
   */
  accessibilityLabel: string;

  /**
   * Callback for when the value of the enum changes.
   */
  onValue?: (value: string) => void;

  /**
   * Callback for when the slider is grabbed or release through a pointer event.
   * Note that this may be called spruriously even if the grabbed state didn't change.
   */
  onGrabOrRelease?: (grabbed: boolean) => void;

  /**
   * Display formatter, if applicable. By default just shows the value.
   */
  displayFormatter?: (value: string) => string;

  /**
   * Component type to use for the value label.
   *
   * Note that this must forward props to an HTML div element.
   */
  ValueLabel: ValueLabel;

  /**
   * Component type to use for the slider.
   *
   * You can use the `useEnumSlider` hook to implement an animated slider.
   */
  Slider: Slider;

  /**
   * Layout of the enum slider.
   */
  layout?: Layout;
};

export const EnumSlider = ({
  value,
  values,
  onValue,
  onGrabOrRelease,
  accessibilityLabel,
  displayFormatter,
  ValueLabel,
  Slider,
  grabbed = false,
  layout = "slider-first",
}: Props) => {
  const index = indexOf(value, values);
  const selectIndex = useCallback(
    (index: number) => {
      if (values[index]) {
        onValue?.(values[index]);
      }
      setTimeout(() => radios.current.get(index)?.focus(), 0);
    },
    [onValue, values],
  );
  const radios = useRef<Map<number, HTMLDivElement>>(new Map());
  useEffect(() => {
    const anyFocused = [...radios.current.values()].some(
      (r) => document.activeElement === r || r.contains(document.activeElement),
    );
    if (anyFocused) {
      radios.current.get(index ?? 0)?.focus();
    }
  }, [index]);
  const slider = (
    <Slider
      key="slider"
      index={index}
      count={values.length}
      selectIndex={selectIndex}
      onGrabOrRelease={onGrabOrRelease}
      grabbed={grabbed}
    />
  );

  const labels = (
    <LabelGroup
      key="labels"
      accessibilityLabel={accessibilityLabel}
      value={value}
      values={values}
      displayFormatter={displayFormatter}
      valueLabel={ValueLabel}
      radios={radios}
      selectIndex={selectIndex}
    ></LabelGroup>
  );

  return (
    <div
      style={{ display: "flex", flexDirection: "row", alignItems: "stretch" }}
    >
      {layout === "slider-first" ? [slider, labels] : [labels, slider]}
    </div>
  );
};

export default EnumSlider;
