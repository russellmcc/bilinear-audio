import Button from "../components/button";
import Slider from "../components/slider";
import type { Rs79Mode } from "../mode";
import title from "./assets/title.svg";
import {
  RS_79_BACKGROUND,
  RS_79_HIGHLIGHT_COLOR,
  RS_79_SWITCH_COLOR,
} from "./constants";
import EnsembleSwitch from "./ensembleSwitch";
import { useRs79State } from "./state";

export type LayoutProps = {
  mode: Rs79Mode;
  setMode: (mode: Rs79Mode) => void;
};

const Layout = (props: LayoutProps) => {
  const rs79 = useRs79State(props);

  return (
    <div
      style={{
        position: "relative",
        width: "400px",
        height: "400px",
        padding: "0px",
        margin: "0px",
        background: RS_79_BACKGROUND,
        whiteSpace: "pre-wrap",
        color: "var(--text-color)",
      }}
    >
      <img
        src={title}
        alt="Organ/Strings 79"
        draggable={false}
        style={{
          display: "block",
          position: "absolute",
          left: "90.1px",
          top: "14.24px",
          width: "287.561px",
          height: "51.517px",
        }}
      />
      <div
        style={{
          position: "absolute",
          right: "21px",
          top: "69px",
          color: RS_79_SWITCH_COLOR,
          fontSize: "18px",
          lineHeight: "21px",
          textAlign: "right",
        }}
      >
        RS-79
      </div>
      <EnsembleSwitch
        value={rs79.ensembleMode}
        onValue={rs79.setEnsembleMode}
      />
      <Slider highlightColor={RS_79_HIGHLIGHT_COLOR} />
      <Button highlightColor={RS_79_HIGHLIGHT_COLOR} />
    </div>
  );
};

export default Layout;
