import { useNextMode } from "../mode";

export type ButtonProps = {
  highlightColor: string;
};

const Button = ({ highlightColor }: ButtonProps) => {
  const triangleSize = 19;
  const sideLength = (triangleSize * Math.sqrt(3)) / 2;

  const nextMode = useNextMode();

  return (
    <button
      style={{
        display: "flex",
        alignItems: "center",
        gap: "4px",
        position: "absolute",
        bottom: "19px",
        right: "21px",
        fontSize: "14px",
      }}
      onClick={nextMode}
      tabIndex={0}
      aria-label="Next mode"
    >
      <span>Next</span>
      <svg
        width={sideLength}
        height={triangleSize}
        viewBox={`0 0 ${sideLength} ${triangleSize}`}
        style={{ display: "block" }}
      >
        <path
          d={`M 1 1 L ${sideLength - 1} ${triangleSize / 2} L 1 ${triangleSize - 1} Z`}
          stroke={highlightColor}
          strokeWidth="1"
          fill="none"
        />
      </svg>
    </button>
  );
};

export default Button;
