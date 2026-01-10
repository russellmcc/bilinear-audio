import {
  Knob as InternalKnob,
  Props as InternalKnobProps,
  DisplayProps,
  LabelProps,
} from "../kit/knob";

type RelevantProps = Omit<InternalKnobProps, "Display" | "Label">;

export type Props = {
  /**
   * The current value of the knob (scaled to 0-100)
   */
  value: number;

  /**
   * True if the knob is grabbed
   */
  grabbed?: boolean;

  /**
   * Callback for when the knob is grabbed or release through a pointer event.
   */
  onGrabOrRelease?: (grabbed: boolean) => void;

  /**
   * Callback for when the value of the knob changes.
   */
  onValue?: (value: number) => void;

  /**
   * The label of the knob. Note this is required for accessibility. To hide the label, set `showLabel` to false.
   */
  label: string;

  /**
   * Whether we should show the label
   */
  showLabel?: boolean;

  /**
   * Value formatter to convert values into strings
   */
  valueFormatter?: (value: number) => string;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;

  /**
   * Value to reset the knob to on reset-to-default gesture (double click)
   */
  defaultValue?: number;
};

const display = ({ value }: DisplayProps) => {
  const radius = 40;
  const center = 50;
  const startAngle = 135;
  const angle = startAngle + (value / 100) * 270;
  const rad = (angle * Math.PI) / 180;
  const startRad = (startAngle * Math.PI) / 180;

  const x = center + radius * Math.cos(rad);
  const y = center + radius * Math.sin(rad);

  const startX = center + radius * Math.cos(startRad);
  const startY = center + radius * Math.sin(startRad);

  const largeArc = angle - startAngle > 180 ? 1 : 0;

  return (
    <svg className="knob-display" viewBox="0 0 100 100">
      <path
        d={`M ${startX} ${startY} A ${radius} ${radius} 0 ${largeArc} 1 ${x} ${y}`}
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
      />
      {
        <line
          x1={center}
          y1={center}
          x2={x}
          y2={y}
          stroke="currentColor"
          strokeWidth="2"
        />
      }
    </svg>
  );
};

const label = ({ label, grabbed, hover, valueLabel }: LabelProps) => (
  <div className="knob-label">{grabbed || hover ? valueLabel : label}</div>
);

export const Knob = ({ ...props }: Props) => (
  <InternalKnob
    {...props}
    Display={display}
    Label={label}
    valueFormatter={(v) => v.toFixed(0)}
  />
);

export default Knob;
