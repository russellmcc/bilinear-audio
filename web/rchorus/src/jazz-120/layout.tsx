import { EnumSlider, useEnumSlider } from "music-ui/kit";
import { useCallback } from "react";
import Button from "../components/button";
import Slider from "../components/slider";
import {
  JAZZ_DEPTH_RANGE,
  JAZZ_RATE_RANGE,
  JAZZ_VIBRATO_DEPTH,
  JAZZ_VIBRATO_RATE,
} from "./constants";
import Knob from "./knob";
import { JazzChorusMode, useJazzChorusState } from "./state";
import title from "./assets/funk-chorus-120.svg";
import { Jazz120Mode } from "../mode";

type ModeSwitchProps = {
  value: JazzChorusMode;
  onValue: (value: JazzChorusMode) => void;
};

const MODE_SWITCH_VALUES = ["vibrato", "chorus"];
const MODE_SWITCH_BALL_SIZE = 21;

const ModeSwitchSlider = ({
  index,
  count,
  selectIndex,
  onGrabOrRelease,
}: EnumSlider.SliderProps) => {
  const { containerRef, ballRef, ball, ...divProps } = useEnumSlider<
    HTMLDivElement,
    HTMLDivElement
  >({
    ballMargin: 1,
    lineSpacing: 46,
    ballSize: MODE_SWITCH_BALL_SIZE,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
  });

  return (
    <div
      {...divProps}
      ref={containerRef}
      style={{
        position: "absolute",
        left: "26px",
        top: "0px",
        width: "21px",
        height: "100px",
      }}
    >
      <div
        style={{
          position: "absolute",
          left: "10px",
          top: "43px",
          width: "1px",
          height: "45px",
          background: "var(--bg-color-jazz-120)",
        }}
      />
      {ball !== undefined && (
        <div
          ref={ballRef}
          style={{
            position: "absolute",
            left: "0px",
            bottom: `${ball.bottom}px`,
            width: `${MODE_SWITCH_BALL_SIZE - 2}px`,
            height: `${MODE_SWITCH_BALL_SIZE - 2}px`,
            border: "1px solid var(--text-color)",
            borderRadius: "4px",
            background: "var(--panel-color-jazz-120)",
          }}
        />
      )}
    </div>
  );
};

const ModeSwitchValueLabel = ({
  label,
  ...props
}: EnumSlider.ValueLabelProps) => {
  const isVibrato = label === "vibrato";
  return (
    <div
      {...props}
      style={{
        position: "absolute",
        left: isVibrato ? "20px" : "2px",
        top: isVibrato ? "0px" : "104px",
        width: isVibrato ? "34px" : "75px",
        fontSize: "18px",
        lineHeight: "22px",
        textAlign: "right",
      }}
    >
      {isVibrato ? "VIB." : "CHORUS"}
    </div>
  );
};

const ModeSwitch = ({ value, onValue }: ModeSwitchProps) => {
  const setMode = useCallback(
    (mode: string) => {
      if (mode === "vibrato" || mode === "chorus") {
        onValue(mode);
      }
    },
    [onValue],
  );

  return (
    <div
      style={{
        position: "absolute",
        left: "280px",
        top: "40px",
        width: "78px",
        height: "126px",
        color: "var(--text-color)",
      }}
    >
      <EnumSlider.EnumSlider
        values={MODE_SWITCH_VALUES}
        value={value}
        onValue={setMode}
        accessibilityLabel="Jazz chorus modulation mode"
        ValueLabel={ModeSwitchValueLabel}
        Slider={ModeSwitchSlider}
      />
    </div>
  );
};

export type LayoutProps = {
  mode: Jazz120Mode;
  setMode: (mode: Jazz120Mode) => void;
};

const Layout = (props: LayoutProps) => {
  const jazz = useJazzChorusState(props);

  return (
    <div
      style={{
        position: "relative",
        width: "400px",
        height: "400px",
        padding: "0px",
        margin: "0px",
        background: "var(--bg-color-jazz-120)",
        whiteSpace: "pre-wrap",
        color: "var(--text-color)",
      }}
    >
      <div
        style={{
          position: "absolute",
          left: "8px",
          top: "8px",
          width: "384px",
          height: "196px",
          borderRadius: "16px",
          background: "var(--frame-color-jazz-120)",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "16px",
          top: "16px",
          width: "368px",
          height: "180px",
          borderRadius: "16px",
          background: "var(--bg-color-jazz-120)",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "24px",
          top: "24px",
          width: "352px",
          height: "164px",
          borderRadius: "8px",
          background: "var(--panel-color-jazz-120)",
        }}
      />
      <div style={{ position: "absolute", left: "21px", top: "40px" }}>
        <Knob
          label="SPEED"
          value={jazz.rate.value}
          set={jazz.rate.set}
          grab={jazz.rate.grab}
          release={jazz.rate.release}
          range={JAZZ_RATE_RANGE}
          active={jazz.controlsActive}
          accessibilityLabel={jazz.rate.info.title}
          defaultValue={JAZZ_VIBRATO_RATE}
        />
      </div>
      <div style={{ position: "absolute", left: "136px", top: "40px" }}>
        <Knob
          label="DEPTH"
          value={jazz.depth.value}
          set={jazz.depth.set}
          grab={jazz.depth.grab}
          release={jazz.depth.release}
          range={JAZZ_DEPTH_RANGE}
          active={jazz.controlsActive}
          accessibilityLabel={jazz.depth.info.title}
          defaultValue={JAZZ_VIBRATO_DEPTH}
        />
      </div>
      <ModeSwitch value={jazz.chorusMode} onValue={jazz.setChorusMode} />
      <div
        style={{
          position: "absolute",
          left: "14.42px",
          top: "234.35px",
          width: "373.789px",
          height: "44.288px",
        }}
      >
        <img
          src={title}
          alt="FUNK CHORUS-120"
          draggable={false}
          style={{
            display: "block",
            width: "100%",
            height: "100%",
          }}
        />
      </div>
      <Slider highlightColor={"var(--highlight-color-jazz-120)"} />
      <Button highlightColor={"var(--highlight-color-jazz-120)"} />
    </div>
  );
};

export default Layout;
