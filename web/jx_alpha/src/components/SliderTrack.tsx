import { BALL_SIZE, TRACK_LENGTH, TRACK_WIDTH } from "./constants";

export type Props = {
  height?: number;
};

export const SliderTrack = ({ height }: Props) => (
  <div
    style={{
      height: `${height ?? TRACK_LENGTH - BALL_SIZE / 2}px`,
      width: `${TRACK_WIDTH}px`,
      position: "absolute",
      top: `${BALL_SIZE / 4}px`,
      left: `${(BALL_SIZE - TRACK_WIDTH) / 2}px`,
      backgroundColor: "var(--darkest-color)",
      borderRadius: `${TRACK_WIDTH}px`,
    }}
  ></div>
);

export default SliderTrack;
