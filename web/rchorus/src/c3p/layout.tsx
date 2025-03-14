import Button from "../components/button";
import Slider from "../components/slider";
import logo from "./assets/logo.svg";

const Layout = () => (
  <div
    style={{
      position: "relative",
      width: "400px",
      height: "400px",
      whiteSpace: "pre-wrap",
      padding: "0px",
      margin: "0px",
      background: "var(--bg-color-c3p)",
    }}
  >
    <div
      style={{
        textAlign: "right",
        marginRight: "21px",
        paddingTop: "11px",
        fontSize: "17.5px",
      }}
    >
      {"Analogue\nModeled\nChorus\nEffect"}
    </div>
    <div
      style={{
        textAlign: "right",
        marginRight: "21px",
        paddingTop: "5px",
      }}
    >
      <img src={logo} draggable={false} />
    </div>
    <Slider highlightColor={"var(--highlight-color-c3p)"} />
    <Button highlightColor={"var(--highlight-color-c3p)"} />
  </div>
);

export default Layout;
