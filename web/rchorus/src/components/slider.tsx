import { useSwitchParam } from "@conformal/plugin";
import { useEnumSlider, EnumSlider } from "music-ui/kit";
import { useCallback } from "react";

const BALL_SIZE = 19;

const InternalSlider = ({
  index,
  count,
  selectIndex: selectIndex,
  onGrabOrRelease,
  highlightColor,
}: EnumSlider.SliderProps & {
  highlightColor: string;
}) => {
  const { containerRef, ballRef, ball, ...divProps } = useEnumSlider<
    HTMLDivElement,
    HTMLDivElement
  >({
    ballMargin: -1,
    lineSpacing: 21,
    ballSize: BALL_SIZE,
    index,
    count,
    selectIndex,
    onGrabOrRelease,
  });

  return (
    <div
      style={{
        position: "relative",
        display: "flex",
        marginRight: "11px",
      }}
    >
      <div style={{ position: "relative" }} {...divProps} ref={containerRef}>
        <div
          style={{
            position: "relative",
            height: "100%",
            width: "21px",
          }}
        >
          <div
            style={{
              width: "1px",
              backgroundColor: "var(--text-color)",
              height: "50%",
              position: "absolute",
              top: "25%",
              left: "10px",
            }}
          ></div>
        </div>
        {ball !== undefined && (
          <div
            style={{
              position: "absolute",
              height: `${BALL_SIZE}px`,
              width: `${BALL_SIZE}px`,
              bottom: `${ball.bottom}px`,
              border: `1px solid ${highlightColor}`,
              borderRadius: "4px",
            }}
            ref={ballRef}
          ></div>
        )}
      </div>
    </div>
  );
};

type SliderProps = {
  highlightColor: string;
};

const Slider = ({ highlightColor }: SliderProps) => {
  const { value: bypassed, set: setBypassed } = useSwitchParam("bypass");
  const enabled = !bypassed;
  const setEnabled = useCallback(
    (enabled: boolean) => {
      setBypassed(!enabled);
    },
    [setBypassed],
  );

  const internalSlider = useCallback(
    (props: EnumSlider.SliderProps) => (
      <InternalSlider {...props} highlightColor={highlightColor} />
    ),
    [highlightColor],
  );

  const internalValueLabel = useCallback(
    ({ label, ...props }: EnumSlider.ValueLabelProps) => (
      <div {...props}>{label}</div>
    ),
    [],
  );

  return (
    <div
      style={{
        position: "absolute",
        bottom: "19px",
        left: "21px",
        fontSize: "14px",
      }}
    >
      <div>Chorus</div>
      <div style={{ marginTop: "11px" }}>
        <EnumSlider.EnumSlider
          values={["On", "Off"]}
          value={enabled ? "On" : "Off"}
          onValue={(v) => {
            setEnabled(v === "On");
          }}
          accessibilityLabel={"Chorus"}
          ValueLabel={internalValueLabel}
          Slider={internalSlider}
        />
      </div>
    </div>
  );
};

export default Slider;
