import Button from "../components/button";
import Slider from "../components/slider";
import bg from "./assets/bg.svg";

const Layout = () => (
  <div
    style={{
      position: "relative",
      width: "400px",
      height: "400px",
      padding: "0px",
      margin: "0px",
      background: `url(${bg}) no-repeat center center`,
      whiteSpace: "pre-wrap",
    }}
  >
    <div
      style={{
        fontSize: "40px",
        fontWeight: "100",
        lineHeight: "40px",
        textAlign: "right",
        marginRight: "21px",
        paddingTop: "74px",
      }}
    >
      {"superdimensional\nchorus"}
    </div>
    <Slider highlightColor={"var(--highlight-color-super-dimension)"} />
    <Button highlightColor={"var(--highlight-color-super-dimension)"} />
  </div>
);

export default Layout;
