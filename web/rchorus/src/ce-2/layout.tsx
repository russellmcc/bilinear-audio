import Button from "../components/button";
import Slider from "../components/slider";
import Knob from "./knob";

const Layout = () => (
  <div
    style={{
      position: "relative",
      width: "400px",
      height: "400px",
      padding: "0px",
      margin: "0px",
      background: "var(--bg-color-ce-2)",
      whiteSpace: "pre-wrap",
      color: "var(--panel-color-ce-2)",
    }}
  >
    <div
      style={{
        position: "absolute",
        top: "175px",
        right: "12px",
        fontSize: "40px",
        fontWeight: "100",
        lineHeight: "40px",
        textAlign: "right",
      }}
    >
      Chorus
    </div>
    <div
      style={{
        position: "absolute",
        top: "213px",
        right: "12px",
        fontSize: "24px",
        fontWeight: "100",
        lineHeight: "24px",
        textAlign: "right",
      }}
    >
      C–2
    </div>
    <div
      style={{
        position: "absolute",
        left: "12px",
        top: "240px",
        width: "376px",
        height: "160px",
        borderRadius: "12px 12px 0px 0px",
        background: "var(--panel-color-ce-2)",
      }}
    />
    <div
      style={{
        position: "absolute",
        left: "64px",
        top: "24px",
      }}
    >
      <Knob label="RATE" param="rate" />
    </div>
    <div
      style={{
        position: "absolute",
        left: "236px",
        top: "24px",
      }}
    >
      <Knob label="DEPTH" param="depth" />
    </div>
    <div style={{ color: "var(--text-color)" }}>
      <Slider highlightColor={"var(--highlight-color-ce-2)"} />
    </div>
    <div style={{ color: "var(--text-color)" }}>
      <Button highlightColor={"var(--highlight-color-ce-2)"} />
    </div>
  </div>
);

export default Layout;
