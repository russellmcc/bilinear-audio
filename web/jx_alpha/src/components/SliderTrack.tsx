import { BALL_SIZE, TRACK_LENGTH } from "./constants";

export type Props = {
  height?: number;
};

const trackWidth = 8;

export const SliderTrack = ({ height }: Props) => (
  <div
    style={{
      height: `${height ?? TRACK_LENGTH - BALL_SIZE / 2}px`,
      width: `${trackWidth}px`,
      position: "absolute",
      top: `${BALL_SIZE / 4}px`,
      left: `${(BALL_SIZE - trackWidth) / 2}px`,
      backgroundColor: "var(--darkest-color)",
      borderRadius: `${trackWidth}px`,
    }}
  ></div>
);

export default SliderTrack;
