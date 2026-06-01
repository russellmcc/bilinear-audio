import Button from "../components/button";
import BypassSlider from "../components/slider";
import type { EnsemblePlusMode } from "../mode";
import HumanVoiceSliders from "./humanVoiceSliders";
import VoiceModeButton from "./voiceModeButton";
import logo from "./assets/ensemble-plus.svg";
import { ENSEMBLE_PLUS_MODES, useEnsemblePlusState } from "./state";

const ENSEMBLE_PLUS_BACKGROUND =
  "linear-gradient(-45.36034602069856deg, #422433 0%, #62374d 99.379%)";
const ENSEMBLE_PLUS_ACCENT_COLOR = "#ff8811";

export type LayoutProps = {
  mode: EnsemblePlusMode;
  setMode: (mode: EnsemblePlusMode) => void;
};

const Layout = (props: LayoutProps) => {
  const ensemblePlus = useEnsemblePlusState(props);

  return (
    <div
      style={{
        position: "relative",
        width: "400px",
        height: "400px",
        padding: "0px",
        margin: "0px",
        background: ENSEMBLE_PLUS_BACKGROUND,
        whiteSpace: "pre-wrap",
        color: "var(--text-color)",
      }}
    >
      <img
        src={logo}
        alt="Ensemble Plus"
        draggable={false}
        style={{
          display: "block",
          position: "absolute",
          left: "147px",
          top: "16px",
          width: "231.86px",
          height: "33.216px",
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "312px",
          top: "49px",
          fontSize: "14px",
          lineHeight: "normal",
          color: ENSEMBLE_PLUS_ACCENT_COLOR,
        }}
      >
        EP-330
      </div>
      <div
        style={{
          position: "absolute",
          left: "29px",
          top: "82px",
          width: "1px",
          height: "189px",
          background: ENSEMBLE_PLUS_ACCENT_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "277px",
          top: "82px",
          width: "1px",
          height: "189px",
          background: ENSEMBLE_PLUS_ACCENT_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "372px",
          top: "82px",
          width: "1px",
          height: "189px",
          background: ENSEMBLE_PLUS_ACCENT_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "0px",
          top: "271px",
          width: "400px",
          height: "1px",
          background: ENSEMBLE_PLUS_ACCENT_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          left: "103px",
          top: "82px",
          fontSize: "14px",
          lineHeight: "normal",
        }}
      >
        HUMAN VOICE
      </div>
      <div
        style={{
          position: "absolute",
          left: "294px",
          top: "82px",
          fontSize: "14px",
          lineHeight: "normal",
        }}
      >
        STRINGS
      </div>
      <HumanVoiceSliders
        active={ensemblePlus.controlsActive}
        vibratoRate={ensemblePlus.vibratoRate}
        humanVoiceDepth={ensemblePlus.humanVoiceDepth}
      />
      <div
        role="group"
        aria-label="Ensemble Plus voice mode"
        style={{
          position: "absolute",
          left: "200px",
          top: "126px",
          width: "155px",
          height: "108px",
        }}
      >
        {ENSEMBLE_PLUS_MODES.map((mode) => (
          <div
            key={mode}
            style={{
              position: "absolute",
              left: mode === "humanVoice" ? "0px" : "94px",
              top: "0px",
            }}
          >
            <VoiceModeButton
              mode={mode}
              active={ensemblePlus.voiceMode === mode}
              onSelect={ensemblePlus.setVoiceMode}
            />
          </div>
        ))}
      </div>
      <BypassSlider highlightColor={ENSEMBLE_PLUS_ACCENT_COLOR} />
      <Button highlightColor={ENSEMBLE_PLUS_ACCENT_COLOR} />
    </div>
  );
};

export default Layout;
