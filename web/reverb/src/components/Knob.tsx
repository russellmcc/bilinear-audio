import { Knob as KnobKit } from "music-ui/kit";
import { lerp, Scale } from "music-ui/util";
import { useKnob } from "plugin-ui";
import { useMemo } from "react";

export type Style = "big" | "small";

export type Props = {
  style: Style;
  param: string;
  label: string;
  scale?: Scale;
};

type DisplayProps = {
  value: number;
  grabbed: boolean;
  hover: boolean;
  style: Style;
};

const Display = ({ value, style }: DisplayProps) => {
  const size = style === "big" ? 120 : 80;

  const radius = size * 0.5 - 0.5;
  const TAU = Math.PI * 2;
  const startPoint = (3 / 8) * TAU;
  const endPoint = (9 / 8) * TAU;
  const arc = `M ${size / 2 + radius * Math.cos(startPoint)} 
                 ${size / 2 + radius * Math.sin(startPoint)} A 
                 ${radius} ${radius} 0 1 1 ${size / 2 + radius * Math.cos(endPoint)} 
                 ${size / 2 + radius * Math.sin(endPoint)}`;

  // Draw a line starting at the center, and reaching the center of the arc, angle determined by value
  const lineAngle = lerp(value / 100, startPoint, endPoint);

  return (
    <svg width={size} height={size}>
      <path d={arc} stroke="var(--sea-color)" strokeWidth="1" fill="none" />
      <line
        x1={size / 2}
        y1={size / 2}
        x2={size / 2 + radius * Math.cos(lineAngle)}
        y2={size / 2 + radius * Math.sin(lineAngle)}
        stroke="var(--sea-color)"
        strokeWidth="1"
      />
    </svg>
  );
};

const SmallDisplay = ({
  value,
  grabbed,
  hover,
}: Omit<DisplayProps, "style">) => (
  <Display value={value} grabbed={grabbed} hover={hover} style="small" />
);

const BigDisplay = ({ value, grabbed, hover }: Omit<DisplayProps, "style">) => (
  <Display value={value} grabbed={grabbed} hover={hover} style="big" />
);

const Label = ({ label, style }: KnobKit.LabelProps & { style: Style }) => (
  <div
    style={{
      textAlign: "center",
      marginTop: style === "big" ? "-22px" : "-16px",
    }}
  >
    {label}
  </div>
);

const SmallLabel = ({ ...props }: KnobKit.LabelProps) => (
  <Label {...props} style="small" />
);

const BigLabel = ({ ...props }: KnobKit.LabelProps) => (
  <Label {...props} style="big" />
);

export const Knob = ({ style, param, label, scale }: Props) => {
  const { ...props } = useKnob({ param, scale });

  const { display, labelComponent } = useMemo(
    () =>
      style === "big"
        ? { display: BigDisplay, labelComponent: BigLabel }
        : { display: SmallDisplay, labelComponent: SmallLabel },
    [style],
  );

  return (
    <KnobKit.Knob
      {...props}
      label={label}
      Display={display}
      Label={labelComponent}
    />
  );
};

export default Knob;
