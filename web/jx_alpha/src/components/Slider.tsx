import { Slider as MusicUISlider, useSlider } from "music-ui/kit";
import { useCallback } from "react";

const ballSize = 20;
const borderWidth = 2;
const trackLength = 100;
const borderRadius = 5;
const dotSize = 4;
const dotOffset = 2;
const trackWidth = 8;
const labelMargin = 5;
const scaleWidth = 30;
const labeledScaleWidth = 10;
const labeledScaleMargin = 2;
const scaleMarginLeft = 0;
const scaleMarginRight = -3;
const tickHeight = 1;
const boldTickHeight = 2;
const boldTicks = [10, 5, 0];

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
            display: "inline-block",
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
            display: "inline-block",
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
    <div>
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
          display: "inline-block",
        }}
        {...containerProps}
      >
        <div
          style={{
            height: `${trackLength - ballSize / 2}px`,
            width: `${trackWidth}px`,
            position: "absolute",
            top: `${ballSize / 4}px`,
            left: `${(ballSize + borderWidth - trackWidth) / 2}px`,
            backgroundColor: "var(--darkest-color)",
            borderRadius: `${trackWidth}px`,
          }}
        ></div>
        <div
          style={{
            height: `${ballSize - borderWidth}px`,
            width: `${ballSize - borderWidth}px`,
            left: "0px",
            position: "absolute",
            bottom: `${ballBottom}px`,
            backgroundColor: "var(--darker-color)",
            borderColor: "var(--darkest-color)",
            borderWidth: `${borderWidth}px`,
            borderStyle: "solid",
            borderRadius: `${borderRadius}px`,
            filter: "drop-shadow(2px 2px 4px rgba(0, 0, 0, 0.25))",
            // Hack for safari to prevent stale rendering
            transform: "translateZ(0)",
          }}
        >
          <div
            style={{
              height: `${dotSize}px`,
              width: `${dotSize}px`,
              left: `${dotOffset}px`,
              top: `${(ballSize - borderWidth - dotSize) / 2}px`,
              position: "absolute",
              backgroundColor: "var(--highlight-red)",
              borderRadius: `${dotSize}px`,
            }}
          ></div>
        </div>
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
