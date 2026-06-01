import Button from "../components/button";
import Slider from "../components/slider";
import type { StringMode } from "../mode";
import ch2o2 from "./assets/ch-2o2.svg";
import titleShell from "./assets/title-shell.svg";
import EnsembleSlider from "./ensembleSlider";
import { useStringState } from "./state";

const STRING_ACCENT_COLOR = "#70a5df";
const STRING_BACKGROUND = "#100007";

export type LayoutProps = {
  mode: StringMode;
  setMode: (mode: StringMode) => void;
};

const Layout = (props: LayoutProps) => {
  const string = useStringState(props);

  return (
    <div
      style={{
        position: "relative",
        width: "400px",
        height: "400px",
        padding: "0px",
        margin: "0px",
        background: STRING_BACKGROUND,
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
          height: "384px",
          border: "3px solid var(--text-color)",
          borderRadius: "16px",
          boxSizing: "border-box",
          pointerEvents: "none",
        }}
      />
      <div
        style={{
          position: "absolute",
          right: "28px",
          top: "0px",
          width: "241px",
          height: "23px",
          background: STRING_BACKGROUND,
        }}
      />
      <img
        src={titleShell}
        alt="Strings"
        draggable={false}
        style={{
          display: "block",
          position: "absolute",
          left: "105px",
          top: "5.18px",
          width: "258px",
          height: "33.99px",
        }}
      />
      <EnsembleSlider
        value={string.ensembleMode}
        onValue={string.setEnsembleMode}
        accentColor={STRING_ACCENT_COLOR}
        backgroundColor={STRING_BACKGROUND}
      />
      <Slider highlightColor={STRING_ACCENT_COLOR} />
      <Button highlightColor={STRING_ACCENT_COLOR} />
      <div
        style={{
          position: "absolute",
          left: "149px",
          top: "378px",
          width: "102px",
          height: "18px",
          background: STRING_BACKGROUND,
        }}
      />
      <img
        src={ch2o2}
        alt="CH-2O2"
        draggable={false}
        style={{
          display: "block",
          position: "absolute",
          left: "158.41px",
          top: "380.02px",
          width: "82.362px",
          height: "12.978px",
        }}
      />
    </div>
  );
};

export default Layout;
