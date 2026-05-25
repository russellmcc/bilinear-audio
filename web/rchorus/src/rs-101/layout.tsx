import Button from "../components/button";
import Slider from "../components/slider";
import ch1o1 from "./assets/ch-1o1.svg";
import strings from "./assets/strings.svg";
import { RS_101_ACCENT_COLOR, RS_101_BACKGROUND } from "./constants";

const Layout = () => (
  <div
    style={{
      position: "relative",
      width: "400px",
      height: "400px",
      padding: "0px",
      margin: "0px",
      background: RS_101_BACKGROUND,
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
        width: "206.5px",
        height: "23px",
        background: RS_101_BACKGROUND,
      }}
    />
    <img
      src={strings}
      alt="Strings"
      draggable={false}
      style={{
        display: "block",
        position: "absolute",
        left: "56.41px",
        top: "6.35px",
        width: "312.996px",
        height: "115.648px",
      }}
    />
    <div
      style={{
        display: "flex",
        gap: "12px",
        position: "absolute",
        left: "78px",
        top: "77px",
        height: "27px",
        alignItems: "center",
      }}
    >
      <div
        style={{
          width: "25px",
          height: "25px",
          borderRadius: "5px",
          background: "var(--text-color)",
        }}
      />
      <div
        style={{
          width: "25px",
          height: "25px",
          borderRadius: "5px",
          background: "var(--text-color)",
        }}
      />
      <div
        style={{
          width: "25px",
          height: "25px",
          borderRadius: "5px",
          background: "var(--text-color)",
        }}
      />
    </div>
    <Slider highlightColor={RS_101_ACCENT_COLOR} />
    <Button highlightColor={RS_101_ACCENT_COLOR} />
    <div
      style={{
        position: "absolute",
        left: "153.5px",
        top: "378px",
        width: "93px",
        height: "17px",
        background: RS_101_BACKGROUND,
      }}
    />
    <img
      src={ch1o1}
      alt="CH-1O1"
      draggable={false}
      style={{
        display: "block",
        position: "absolute",
        left: "166.01px",
        top: "379.04px",
        width: "65.776px",
        height: "12.96px",
      }}
    />
  </div>
);

export default Layout;
