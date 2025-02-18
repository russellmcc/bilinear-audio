import { useSwitchParam } from "@conformal/plugin";
import {
  useEnumSlider as useAnimatedEnumSlider,
  EnumSlider as Kit,
} from "music-ui/kit";
import { useGrabbed } from "plugin-ui";
import { useCallback, useMemo } from "react";

const BALL_SIZE = 17;
const BORDER_WIDTH = 1;
const LINE_SPACING = 35;
const SLIDER_WIDTH = 19;
const BALL_MARGIN = 1;
const TEXT_PADDING = 6;

const Slider = ({
  index,
  count,
  selectIndex: selectIndex,
  onGrabOrRelease,
}: Kit.SliderProps) => {
  const { containerRef, ballRef, ball, ...props } = useAnimatedEnumSlider<
    HTMLDivElement,
    HTMLDivElement
  >({
    ballMargin: BALL_MARGIN,
    lineSpacing: LINE_SPACING,
    ballSize: BALL_SIZE,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
  });

  return (
    <div style={{ position: "relative" }} {...props} ref={containerRef}>
      <div
        style={{
          position: "relative",
          height: "100%",
          width: `${SLIDER_WIDTH}px`,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
        }}
      >
        <div style={{ height: "26px" }}></div>
        <div
          style={{
            width: "1px",
            backgroundColor: "var(--sea-color)",
            flexGrow: 1,
          }}
        ></div>
        <div style={{ height: "9px" }}></div>
      </div>
      {ball !== undefined && (
        <div
          style={{
            position: "absolute",
            height: `${BALL_SIZE - BORDER_WIDTH * 2}px`,
            width: `${BALL_SIZE - BORDER_WIDTH * 2}px`,
            left: `${BALL_MARGIN}px`,
            bottom: `${ball.bottom}px`,
            border: `${BORDER_WIDTH}px solid var(--sea-color)`,
            borderRadius: `100px`,
          }}
          ref={ballRef}
        ></div>
      )}
    </div>
  );
};

export const BypassSlider = () => {
  const {
    value: bypassed,
    set: setBypassed,
    grab,
    release,
  } = useSwitchParam("bypass");
  const { grabbed, onGrabOrRelease } = useGrabbed({ grab, release });
  const enabled = !bypassed;
  const setEnabled = useCallback(
    (enabled: boolean) => {
      setBypassed(!enabled);
    },
    [setBypassed],
  );
  const values = useMemo(() => ["on", "off"], []);
  return (
    <div>
      <Kit.EnumSlider
        values={values}
        value={enabled ? "on" : "off"}
        onValue={(v) => {
          setEnabled(v === "on");
        }}
        accessibilityLabel={"Enabled"}
        grabbed={grabbed}
        onGrabOrRelease={onGrabOrRelease}
        // eslint-disable-next-line prefer-arrow-functions/prefer-arrow-functions
        ValueLabel={function ValueLabel({ label, ref, ...props }) {
          return (
            <div
              style={{
                height: LINE_SPACING - TEXT_PADDING,
                paddingTop: TEXT_PADDING,
                marginLeft: "3px",
              }}
              ref={ref}
              {...props}
            >
              {label}
            </div>
          );
        }}
        Slider={Slider}
      />
    </div>
  );
};

export default BypassSlider;
