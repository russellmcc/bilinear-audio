import { Slider as MusicUISlider, useSlider } from "music-ui/kit";
import { useCallback } from "react";
import {
  ballSize,
  boldTickHeight,
  boldTicks,
  labeledScaleMargin,
  labeledScaleWidth,
  labelMargin,
  scaleMarginLeft,
  scaleMarginRight,
  scaleWidth,
  tickHeight,
  trackLength,
} from "./sliderConstants";
import SliderBall from "./SliderBall";
import SliderTrack from "./SliderTrack";

export type ScaleType = "none" | "continuation" | "labeled";

const tickToMiddle = (tick: number) => {
  const inverted = 10 - tick;
  return inverted * ((trackLength - ballSize - 1) / 10) + ballSize / 2;
};

const TickLabel = ({ tick }: { tick: number }) => (
  <div
    style={{
      width: `${scaleWidth - labeledScaleWidth - labeledScaleMargin}px`,
      left: "0px",
      top: `calc(${tickToMiddle(tick) + 1}px - 0.5rem)`,
      marginRight: `${labeledScaleMargin}px`,
      position: "absolute",
      textAlign: "right",
    }}
  >
    {tick}
  </div>
);

const Scale = ({ type }: { type: ScaleType }): React.ReactNode => {
  switch (type) {
    case "none":
      return null;
    case "continuation":
      return (
        <div
          style={{
            height: `${trackLength}px`,
            width: `${scaleWidth}px`,
            position: "relative",
            marginLeft: `${scaleMarginLeft}px`,
            marginRight: `${scaleMarginRight}px`,
            zIndex: -1,
          }}
        >
          {Array.from(Array(11).keys())

            .map((_, i) => {
              const height = boldTicks.includes(i)
                ? boldTickHeight
                : tickHeight;
              return (
                <div
                  key={i}
                  style={{
                    height: `${height}px`,
                    width: `${scaleWidth}px`,
                    position: "absolute",
                    backgroundColor: "var(--fg-color)",
                    top: `${tickToMiddle(i) - height / 2}px`,
                  }}
                ></div>
              );
            })}
        </div>
      );
    case "labeled":
      return (
        <div
          style={{
            height: `${trackLength}px`,
            width: `${scaleWidth}px`,
            position: "relative",
            marginLeft: `${scaleMarginLeft}px`,
            marginRight: `${scaleMarginRight}px`,
            zIndex: -1,
          }}
        >
          <TickLabel tick={10} />
          <TickLabel tick={5} />
          <TickLabel tick={0} />
          {Array.from(Array(11).keys()).map((_, i) => {
            const height = boldTicks.includes(i) ? boldTickHeight : tickHeight;
            return (
              <div
                key={i}
                style={{
                  height: `${height}px`,
                  width: `${labeledScaleWidth}px`,
                  left: `${scaleWidth - labeledScaleWidth}px`,
                  position: "absolute",
                  backgroundColor: "var(--fg-color)",
                  top: `${tickToMiddle(i) - height / 2}px`,
                }}
              ></div>
            );
          })}
        </div>
      );
  }
};

type InternalSliderProps = MusicUISlider.SliderProps & { scale?: ScaleType };
const InternalSlider = ({
  value,
  onValue,
  onGrabOrRelease,
  scale = "none",
}: InternalSliderProps) => {
  const { containerProps, ballBottom } = useSlider({
    value,
    ballMargin: 0,
    ballSize,
    onGrabOrRelease,
    onValue,
  });
  return (
    <div style={{ display: "flex" }}>
      <Scale type={scale} />
      <div
        style={{
          height: `${trackLength}px`,
          width: `${ballSize}px`,
          position: "relative",
          marginLeft:
            scale === "none"
              ? `${scaleWidth + scaleMarginLeft + scaleMarginRight}px`
              : "0px",
        }}
        {...containerProps}
      >
        <SliderTrack />
        <SliderBall bottom={ballBottom} />
      </div>
    </div>
  );
};
const Label = ({ label }: MusicUISlider.LabelProps) => (
  <div
    style={{
      width: `${scaleWidth + scaleMarginLeft + scaleMarginRight + ballSize}px`,
      textAlign: "right",
      marginBottom: `${labelMargin}px`,
    }}
  >
    {label}
  </div>
);

export type Props = {
  /**
   * The current value of the slider
   */
  value: number;

  /**
   * The label of the slider
   */
  label: string;

  /**
   * Callback for when the value of the slider changes.
   */
  onValue: (value: number) => void;

  /**
   * Callback for when the slider is grabbed
   */
  grab: () => void;

  /**
   * Callback for when the slider is released
   */
  release: () => void;

  /**
   * The default value of the slider
   */
  defaultValue?: number;

  /**
   * The visual scale to display to the left of the slider.
   */
  scale?: ScaleType;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;
};

export const Slider = (props: Props) => {
  const { grab, release, scale } = props;
  const onGrabOrRelease = useCallback(
    (grabbed: boolean) => {
      if (grabbed) {
        grab();
      } else {
        release();
      }
    },
    [grab, release],
  );
  const sliderWithScale = useCallback(
    (args: MusicUISlider.SliderProps) => (
      <InternalSlider {...args} scale={scale} />
    ),
    [scale],
  );
  return (
    <MusicUISlider.Slider
      Slider={sliderWithScale}
      Label={Label}
      onGrabOrRelease={onGrabOrRelease}
      valueFormatter={(value) => value.toFixed(0)}
      showLabel="before"
      {...props}
    />
  );
};

export default Slider;
