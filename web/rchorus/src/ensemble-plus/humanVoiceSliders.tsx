import HumanVoiceSlider from "./humanVoiceSlider";
import {
  HUMAN_VOICE_DEFAULT_ENS_DEPTH,
  HUMAN_VOICE_DEFAULT_RATE,
} from "./preset";

const HUMAN_VOICE_RATE_RANGE = [0.7, 7] as const;
const HUMAN_VOICE_ENS_DEPTH_RANGE = [0, 100] as const;
const TICK_SVG_TOP = 115;
const TICK_SVG_HEIGHT = 122;
const TICK_Y_VALUES = [
  1, 13.5, 25.5, 37.5, 49.5, 61, 73.5, 85.5, 97.5, 109.5, 121,
];

type SliderControl = {
  value: number;
  set: (value: number) => void;
  grab: () => void;
  release: () => void;
  info: {
    title: string;
  };
};

export type HumanVoiceSlidersProps = {
  active: boolean;
  vibratoRate: SliderControl;
  humanVoiceDepth: SliderControl;
};

type TickColumnProps = {
  left: number;
  width: number;
  variant: "scale" | "slider";
};

const TickColumn = ({ left, width, variant }: TickColumnProps) => (
  <svg
    aria-hidden="true"
    viewBox={`0 0 ${width} ${TICK_SVG_HEIGHT}`}
    style={{
      position: "absolute",
      left: `${left}px`,
      top: `${TICK_SVG_TOP}px`,
      width: `${width}px`,
      height: `${TICK_SVG_HEIGHT}px`,
      pointerEvents: "none",
      overflow: "visible",
    }}
  >
    {TICK_Y_VALUES.map((y, index) => {
      const isMajor = index === 0 || index === 5 || index === 10;
      const x1 =
        variant === "scale" && !isMajor
          ? 5
          : variant === "scale" && index === 0
            ? 1
            : 0;
      const x2 = variant === "scale" && index === 5 ? width - 1 : width;
      return (
        <line
          key={index}
          x1={x1}
          y1={y}
          x2={x2}
          y2={y}
          stroke="currentColor"
          strokeWidth={isMajor ? 2 : 1}
        />
      );
    })}
  </svg>
);

const ScaleNumber = ({
  value,
  left,
  centerY,
}: {
  value: number;
  left: number;
  centerY: number;
}) => (
  <div
    aria-hidden="true"
    style={{
      position: "absolute",
      left: `${left}px`,
      top: `${centerY}px`,
      transform: "translate(-50%, -50%)",
      fontSize: "14px",
      lineHeight: "14px",
      color: "white",
      textAlign: "center",
      pointerEvents: "none",
    }}
  >
    {value}
  </div>
);

const HumanVoiceSliders = ({
  active,
  vibratoRate,
  humanVoiceDepth,
}: HumanVoiceSlidersProps) => (
  <div
    style={{
      position: "absolute",
      left: "0px",
      top: "0px",
      width: "184px",
      height: "267px",
      opacity: active ? 1 : 0.38,
    }}
  >
    <ScaleNumber value={10} left={45} centerY={116} />
    <ScaleNumber value={5} left={48.5} centerY={176} />
    <ScaleNumber value={0} left={48.5} centerY={236} />
    <TickColumn left={56} width={17} variant="scale" />
    <TickColumn left={97} width={37} variant="slider" />
    <TickColumn left={158} width={26} variant="slider" />
    <div style={{ position: "absolute", left: "64px", top: "117px" }}>
      <HumanVoiceSlider
        label="RATE"
        value={vibratoRate.value}
        set={vibratoRate.set}
        grab={vibratoRate.grab}
        release={vibratoRate.release}
        range={HUMAN_VOICE_RATE_RANGE}
        active={active}
        accessibilityLabel={vibratoRate.info.title}
        defaultValue={HUMAN_VOICE_DEFAULT_RATE}
        trackLeft={17}
      />
    </div>
    <div style={{ position: "absolute", left: "126px", top: "117px" }}>
      <HumanVoiceSlider
        label="DEPTH"
        value={humanVoiceDepth.value}
        set={humanVoiceDepth.set}
        grab={humanVoiceDepth.grab}
        release={humanVoiceDepth.release}
        range={HUMAN_VOICE_ENS_DEPTH_RANGE}
        active={active}
        accessibilityLabel={humanVoiceDepth.info.title}
        defaultValue={HUMAN_VOICE_DEFAULT_ENS_DEPTH}
        trackLeft={16}
      />
    </div>
  </div>
);

export default HumanVoiceSliders;
