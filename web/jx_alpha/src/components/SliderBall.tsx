import { ballSize } from "./sliderConstants";

const borderWidth = 2;
const borderRadius = 5;
export const dotSize = 4;
export const dotOffset = 2;

export type Props = {
  bottom: number;
  ref?: React.RefObject<HTMLDivElement | null>;
};
export const SliderBall = ({ bottom, ref }: Props) => (
  <div
    style={{
      height: `${ballSize - 2 * borderWidth}px`,
      width: `${ballSize - 2 * borderWidth}px`,
      left: "0px",
      position: "absolute",
      bottom: `${bottom}px`,
      backgroundColor: "var(--darker-color)",
      borderColor: "var(--darkest-color)",
      borderWidth: `${borderWidth}px`,
      borderStyle: "solid",
      borderRadius: `${borderRadius}px`,
      filter: "drop-shadow(2px 2px 4px rgba(0, 0, 0, 0.25))",
      // Hack for safari to prevent stale rendering
      transform: "translateZ(0)",
    }}
    ref={ref}
  >
    <div
      style={{
        height: `${dotSize}px`,
        width: `${dotSize}px`,
        left: `${dotOffset}px`,
        top: `${(ballSize - dotSize) / 2 - borderWidth}px`,
        position: "absolute",
        backgroundColor: "var(--highlight-red)",
        borderRadius: `${dotSize}px`,
      }}
    ></div>
  </div>
);

export default SliderBall;
