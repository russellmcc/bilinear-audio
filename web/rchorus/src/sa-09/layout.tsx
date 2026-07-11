import { useCallback } from "react";
import Button from "../components/button";
import Slider from "../components/slider";
import type { Sa09Mode } from "../mode";
import chorusO9 from "./assets/chorus-o9.svg";
import Knob from "./knob";
import ArrowEnumSwitch from "./modeSwitch";
import { SA_09_CHORUS_MODES, SA_09_ROUTING_MODES, useSa09State } from "./state";
import { SA_09_VIBRATO_RATE, SA_09_VIBRATO_RATE_RANGE } from "./preset";

const SA_09_BACKGROUND = "#0d1113";
const SA_09_ACCENT = "#f17105";

const colorBars = [
  {
    id: "pink",
    left: 148,
    height: 40,
    from: "#d81159",
    to: "#bd0f4e",
  },
  {
    id: "orange",
    left: 198,
    height: 80,
    from: "#f17105",
    to: "#c85e04",
  },
  {
    id: "yellow",
    left: 248,
    height: 120,
    from: "#fee440",
    to: "#d1be48",
  },
  {
    id: "cyan",
    left: 298,
    height: 160,
    from: "#05a8aa",
    to: "#04898b",
  },
  {
    id: "teal",
    left: 348,
    height: 200,
    from: "#0e7c7b",
    to: "#0b6564",
  },
] as const;

export type LayoutProps = {
  mode: Sa09Mode;
  setMode: (mode: Sa09Mode) => void;
};

const Layout = (props: LayoutProps) => {
  const sa09 = useSa09State(props);
  const setChorusMode = useCallback(
    (value: string) => {
      if (value === "vibrato" || value === "chorus") {
        sa09.setChorusMode(value);
      }
    },
    [sa09],
  );
  const setRoutingMode = useCallback(
    (value: string) => {
      if (value === "I" || value === "II") {
        sa09.setRoutingMode(value);
      }
    },
    [sa09],
  );
  const formatChorusMode = useCallback(
    (value: string) => (value === "vibrato" ? "Vibrato" : "Chorus"),
    [],
  );
  const formatRoutingMode = useCallback((value: string) => `Mode ${value}`, []);

  return (
    <div
      style={{
        position: "relative",
        width: "400px",
        height: "400px",
        padding: "0px",
        margin: "0px",
        background: SA_09_BACKGROUND,
        whiteSpace: "pre-wrap",
        color: "var(--text-color)",
        overflow: "hidden",
      }}
    >
      <div
        style={{
          position: "absolute",
          left: "0px",
          top: "75.5px",
          width: "49px",
          height: "1px",
          background: "var(--text-color)",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "107px",
          top: "75.5px",
          width: "293px",
          height: "1px",
          background: "var(--text-color)",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "37px",
          top: "55px",
        }}
      >
        <Knob
          value={sa09.vibratoRate.value}
          set={sa09.vibratoRate.set}
          grab={sa09.vibratoRate.grab}
          release={sa09.vibratoRate.release}
          range={SA_09_VIBRATO_RATE_RANGE}
          active={sa09.controlsActive}
          accessibilityLabel={sa09.vibratoRate.info.title}
          defaultValue={SA_09_VIBRATO_RATE}
        />
      </div>
      <img
        src={chorusO9}
        alt="Chorus O9"
        draggable={false}
        style={{
          display: "block",
          position: "absolute",
          left: "150.265px",
          top: "16.738px",
          width: "234.863px",
          height: "38.262px",
        }}
      />
      {colorBars.map(({ id, left, height, from, to }) => (
        <div
          key={id}
          style={{
            position: "absolute",
            left: `${left}px`,
            top: "96px",
            width: "40px",
            height: `${height}px`,
            borderRadius: "20px",
            background: `linear-gradient(180deg, ${from} 0%, ${to} 100%)`,
          }}
        />
      ))}
      <ArrowEnumSwitch
        values={SA_09_CHORUS_MODES}
        value={sa09.chorusMode}
        onValue={setChorusMode}
        accessibilityLabel="SA-09 chorus mode"
        displayFormatter={formatChorusMode}
        topLabel="Vibrato"
        left={21}
        labelWidth={48}
      />
      <ArrowEnumSwitch
        values={SA_09_ROUTING_MODES}
        value={sa09.routingMode}
        onValue={setRoutingMode}
        accessibilityLabel="SA-09 routing mode"
        displayFormatter={formatRoutingMode}
        topLabel="Mode I"
        left={84}
        labelWidth={49}
      />
      <Slider highlightColor={SA_09_ACCENT} />
      <Button highlightColor={SA_09_ACCENT} />
    </div>
  );
};

export default Layout;
