import Button from "../components/button";
import Slider from "../components/slider";
import type { Ju60Mode } from "../mode";
import {
  JU_60_ACCENT_COLOR,
  JU_60_BACKGROUND,
  JU_60_BUTTON_MODES,
  type Ju60ButtonMode,
} from "./constants";
import ModeButton from "./modeButton";
import { useJu60State } from "./state";

export type LayoutProps = {
  mode: Ju60Mode;
  setMode: (mode: Ju60Mode) => void;
};

const Layout = (props: LayoutProps) => {
  const ju60 = useJu60State(props);
  const selectMode = (mode: Ju60ButtonMode) => {
    ju60.setButtonMode(mode);
    const input = document.getElementById(`ju-60-mode-${mode}`);
    if (input instanceof HTMLElement) {
      input.focus();
    }
  };
  const selectModeByOffset = (mode: Ju60ButtonMode, offset: number) => {
    const currentIndex = JU_60_BUTTON_MODES.indexOf(mode);
    const nextIndex =
      (currentIndex + offset + JU_60_BUTTON_MODES.length) %
      JU_60_BUTTON_MODES.length;
    const nextMode = JU_60_BUTTON_MODES[nextIndex];
    if (nextMode === undefined) {
      return;
    }
    selectMode(nextMode);
  };

  return (
    <div
      style={{
        position: "relative",
        width: "400px",
        height: "400px",
        padding: "0px",
        margin: "0px",
        background: JU_60_BACKGROUND,
        whiteSpace: "pre-wrap",
        color: "var(--text-color)",
      }}
    >
      <div
        style={{
          position: "absolute",
          left: "0px",
          top: "20px",
          width: "400px",
          height: "40px",
          background: JU_60_ACCENT_COLOR,
        }}
      />
      <div
        style={{
          position: "absolute",
          right: "19px",
          top: "25px",
          fontSize: "32px",
          lineHeight: "32px",
          color: "#ffffff",
          letterSpacing: "0.5px",
        }}
      >
        SYNTHESIZER CHORUS
      </div>
      <div
        role="radiogroup"
        aria-label="Synthesizer chorus mode"
        style={{
          position: "absolute",
          left: "30px",
          top: "97px",
          width: "340px",
          height: "184px",
          margin: "0px",
          padding: "0px",
          border: "0px",
          minInlineSize: "0px",
        }}
      >
        <div
          style={{
            display: "flex",
            justifyContent: "center",
            width: "100%",
          }}
        >
          {JU_60_BUTTON_MODES.map((mode) => (
            <ModeButton
              key={mode}
              mode={mode}
              active={mode === ju60.buttonMode}
              onSelect={selectMode}
              onSelectByOffset={selectModeByOffset}
            />
          ))}
        </div>
      </div>
      <Slider highlightColor={JU_60_ACCENT_COLOR} />
      <Button highlightColor={JU_60_ACCENT_COLOR} />
    </div>
  );
};

export default Layout;
