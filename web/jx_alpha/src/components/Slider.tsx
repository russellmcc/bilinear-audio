import { Slider as MusicUISlider, useSlider } from "music-ui/kit";
import { useCallback } from "react";
import { BALL_SIZE, LABEL_MARGIN, TRACK_LENGTH } from "./constants";
import SliderBall from "./SliderBall";
import SliderTrack from "./SliderTrack";
import useOnGrabOrRelease from "./useGrabOrRelease";

export type ScaleType = "none" | "continuation" | "labeled";

const TICK_HEIGHT = 1;
const BOLD_TICK_HEIGHT = 2;
const BOLD_TICKS: readonly number[] = [10, 5, 0];
const SCALE_WIDTH = 17;
const LABEL_WIDTH = 10;
const LABELED_SCALE_WIDTH = 10;
const LABELED_SCALE_MARGIN = 4;
const SCALE_MARGIN_LEFT = -3;
const SCALE_MARGIN_RIGHT = -3;

const tickToMiddle = (tick: number) => {
  const inverted = 10 - tick;
  return inverted * ((TRACK_LENGTH - BALL_SIZE - 1) / 10) + BALL_SIZE / 2;
};

const TickLabel = ({ tick }: { tick: number }) => (
  <div
    style={{
      width: `${LABEL_WIDTH}px`,
      left: "0px",
      top: `calc(${tickToMiddle(tick) + 1}px - 0.5rem)`,
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
            height: `${TRACK_LENGTH}px`,
            width: `${SCALE_WIDTH}px`,
            position: "relative",
            marginLeft: `${SCALE_MARGIN_LEFT}px`,
            marginRight: `${SCALE_MARGIN_RIGHT}px`,
            zIndex: -1,
          }}
        >
          {Array.from(Array(11).keys())

            .map((_, i) => {
              const height = BOLD_TICKS.includes(i)
                ? BOLD_TICK_HEIGHT
                : TICK_HEIGHT;
              return (
                <div
                  key={i}
                  style={{
                    height: `${height}px`,
                    width: `${SCALE_WIDTH}px`,
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
            height: `${TRACK_LENGTH}px`,
            width: `${LABEL_WIDTH + LABELED_SCALE_MARGIN + LABELED_SCALE_WIDTH}px`,
            position: "relative",
            marginLeft: `${SCALE_MARGIN_LEFT}px`,
            marginRight: `${SCALE_MARGIN_RIGHT}px`,
            zIndex: -1,
          }}
        >
          <TickLabel tick={10} />
          <TickLabel tick={5} />
          <TickLabel tick={0} />
          {Array.from(Array(11).keys()).map((_, i) => {
            const height = BOLD_TICKS.includes(i)
              ? BOLD_TICK_HEIGHT
              : TICK_HEIGHT;
            return (
              <div
                key={i}
                style={{
                  height: `${height}px`,
                  width: `${LABELED_SCALE_WIDTH}px`,
                  left: `${LABEL_WIDTH + LABELED_SCALE_MARGIN}px`,
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
    ballSize: BALL_SIZE,
    onGrabOrRelease,
    onValue,
  });
  return (
    <div style={{ display: "flex" }}>
      <Scale type={scale} />
      <div
        style={{
          height: `${TRACK_LENGTH}px`,
          width: `${BALL_SIZE}px`,
          position: "relative",
          marginLeft:
            scale === "none"
              ? `${SCALE_WIDTH + SCALE_MARGIN_LEFT + SCALE_MARGIN_RIGHT}px`
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
      textAlign: "right",
      marginBottom: `${LABEL_MARGIN}px`,
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
  const onGrabOrRelease = useOnGrabOrRelease({ grab, release });
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
