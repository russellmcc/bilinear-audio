import Button from "../components/button";
import Slider from "../components/slider";
import cp4 from "./assets/cp-4.svg";
import compufonicChorus from "./assets/compufonic-chorus.svg";
import ellipse21 from "./assets/ellipse-21.svg";

const CP_4_ACCENT_COLOR = "#12eaea";
const CP_4_BACKGROUND = "linear-gradient(180deg, #0d1012 0%, #151b1e 100%)";

const colorBarSegments = [
  { id: "cyan-1", from: "#12eaea", to: "#0fc2c2" },
  { id: "white-1", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "white-2", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "white-3", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "white-4", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "cyan-2", from: "#12eaea", to: "#0fc2c2" },
  { id: "cyan-3", from: "#12eaea", to: "#0fc2c2" },
  { id: "cyan-4", from: "#12eaea", to: "#0fc2c2" },
  { id: "cyan-5", from: "#12eaea", to: "#0fc2c2" },
  { id: "blue-1", from: "#6a8695", to: "#586f7c" },
  { id: "red-1", from: "#e00038", to: "#991f3d" },
  { id: "yellow-1", from: "#ffd275", to: "#c7b185" },
  { id: "white-5", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "white-6", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "white-7", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "white-8", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "blue-2", from: "#6a8695", to: "#586f7c" },
  { id: "blue-3", from: "#6a8695", to: "#586f7c" },
  { id: "blue-4", from: "#6a8695", to: "#586f7c" },
  { id: "blue-5", from: "#6a8695", to: "#586f7c" },
  { id: "cyan-6", from: "#12eaea", to: "#0fc2c2" },
  { id: "cyan-7", from: "#12eaea", to: "#0fc2c2" },
  { id: "white-9", from: "#d6dbd2", to: "#c2c3c1" },
  { id: "yellow-2", from: "#ffd275", to: "#c7b185" },
  { id: "yellow-3", from: "#ffd275", to: "#c7b185" },
  { id: "yellow-4", from: "#ffd275" },
];

const Layout = () => (
  <div
    style={{
      position: "relative",
      width: "400px",
      height: "400px",
      padding: "0px",
      margin: "0px",
      background: CP_4_BACKGROUND,
      whiteSpace: "pre-wrap",
      color: "var(--text-color)",
      overflow: "hidden",
    }}
  >
    <img
      src={ellipse21}
      alt=""
      draggable={false}
      style={{
        display: "block",
        position: "absolute",
        left: "44px",
        top: "0px",
        width: "405px",
        height: "405px",
      }}
    />
    <img
      src={compufonicChorus}
      alt="COMPUFONIC CHORUS"
      draggable={false}
      style={{
        display: "block",
        position: "absolute",
        left: "110.22px",
        top: "137.84px",
        width: "264.164px",
        height: "88.484px",
      }}
    />
    <img
      src={cp4}
      alt="CP-4"
      draggable={false}
      style={{
        display: "block",
        position: "absolute",
        left: "17.25px",
        top: "23.69px",
        width: "67.91px",
        height: "15.625px",
      }}
    />
    <div
      style={{
        position: "absolute",
        left: "0px",
        top: "250px",
        width: "416px",
        height: "12px",
        display: "flex",
        overflow: "hidden",
        transform: "rotate(0.28deg)",
        transformOrigin: "left top",
      }}
    >
      {colorBarSegments.map(({ id, from, to }) => (
        <div
          key={id}
          style={{
            width: "16px",
            height: "12px",
            flexShrink: 0,
            background:
              to === undefined
                ? from
                : `linear-gradient(180deg, ${from} 0%, ${to} 100%)`,
          }}
        />
      ))}
    </div>
    <Slider highlightColor={CP_4_ACCENT_COLOR} />
    <Button highlightColor={CP_4_ACCENT_COLOR} />
  </div>
);

export default Layout;
