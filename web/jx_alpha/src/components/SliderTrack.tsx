import { ballSize, trackLength } from "./sliderConstants";

export type Props = {
  height?: number;
};

const trackWidth = 8;

export const SliderTrack = ({ height }: Props) => (
  <div
    style={{
      height: `${height ?? trackLength - ballSize / 2}px`,
      width: `${trackWidth}px`,
      position: "absolute",
      top: `${ballSize / 4}px`,
      left: `${(ballSize - trackWidth) / 2}px`,
      backgroundColor: "var(--darkest-color)",
      borderRadius: `${trackWidth}px`,
    }}
  ></div>
);

export default SliderTrack;
