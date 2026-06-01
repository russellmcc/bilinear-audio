import Button from "../components/button";
import Slider from "../components/slider";
import separator from "./assets/separator.svg";
import scc350 from "./assets/scc-350.svg";
import titleRule from "./assets/title-rule.svg";
import vocoder from "./assets/vocoder.svg";

const SVC_350_BACKGROUND =
  "linear-gradient(135deg, #230611 0%, #120309 100%)";
const SVC_350_ACCENT_COLOR = "#f7717d";

const Layout = () => (
  <div
    style={{
      position: "relative",
      width: "400px",
      height: "400px",
      padding: "0px",
      margin: "0px",
      background: SVC_350_BACKGROUND,
      whiteSpace: "pre-wrap",
      color: "var(--text-color)",
      overflow: "hidden",
    }}
  >
    <div
      style={{
        position: "absolute",
        left: "104px",
        top: "0px",
        width: "1px",
        height: "400px",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <div style={{ flex: "none", transform: "rotate(90.14deg)" }}>
        <div
          style={{ position: "relative", width: "400.001px", height: "0px" }}
        >
          <img
            src={separator}
            alt=""
            draggable={false}
            style={{
              display: "block",
              position: "absolute",
              left: "0px",
              top: "-4px",
              width: "400.001px",
              height: "4px",
            }}
          />
        </div>
      </div>
    </div>
    <img
      src={vocoder}
      alt="VOCODER"
      draggable={false}
      style={{
        display: "block",
        position: "absolute",
        left: "174.13px",
        top: "171.86px",
        width: "152.734px",
        height: "22.391px",
      }}
    />
    <img
      src={titleRule}
      alt=""
      draggable={false}
      style={{
        display: "block",
        position: "absolute",
        left: "125px",
        top: "204px",
        width: "279px",
        height: "1px",
      }}
    />
    <img
      src={scc350}
      alt="SCC-350"
      draggable={false}
      style={{
        display: "block",
        position: "absolute",
        left: "210px",
        top: "216.86px",
        width: "116.047px",
        height: "22.391px",
      }}
    />
    <Slider highlightColor={SVC_350_ACCENT_COLOR} />
    <Button highlightColor={SVC_350_ACCENT_COLOR} />
  </div>
);

export default Layout;
