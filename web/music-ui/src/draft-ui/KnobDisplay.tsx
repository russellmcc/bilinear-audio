import { DisplayProps } from "../kit/knob";

export const KnobDisplay = ({ value }: DisplayProps) => {
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

export default KnobDisplay;
