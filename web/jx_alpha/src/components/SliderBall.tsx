import {
  BALL_SIZE,
  BORDER_WIDTH,
  DOT_OFFSET,
  DOT_SIZE,
  DROP_SHADOW_FILTER,
} from "./constants";

const borderRadius = 5;

export type Props = {
  bottom: number;
  ref?: React.RefObject<HTMLDivElement | null>;
};
export const SliderBall = ({ bottom, ref }: Props) => (
  <div
    style={{
      height: `${BALL_SIZE - 2 * BORDER_WIDTH}px`,
      width: `${BALL_SIZE - 2 * BORDER_WIDTH}px`,
      left: "0px",
      position: "absolute",
      bottom: `${bottom}px`,
      backgroundColor: "var(--darker-color)",
      borderColor: "var(--darkest-color)",
      borderWidth: `${BORDER_WIDTH}px`,
      borderStyle: "solid",
      borderRadius: `${borderRadius}px`,
      filter: DROP_SHADOW_FILTER,
      // Hack for safari to prevent stale rendering
      transform: "translateZ(0)",
    }}
    ref={ref}
  >
    <div
      style={{
        height: `${DOT_SIZE}px`,
        width: `${DOT_SIZE}px`,
        left: `${DOT_OFFSET}px`,
        top: `${(BALL_SIZE - DOT_SIZE) / 2 - BORDER_WIDTH}px`,
        position: "absolute",
        backgroundColor: "var(--highlight-red)",
        borderRadius: `${DOT_SIZE}px`,
      }}
    ></div>
  </div>
);

export default SliderBall;
