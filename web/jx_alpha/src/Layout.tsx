import DCO1 from "./layout/DCO1";
import DCO2 from "./layout/DCO2";
import DCOMod from "./layout/DCOMod";
import ENV from "./layout/ENV";
import Mixer from "./layout/Mixer";
import VCF from "./layout/VCF";
import logo from "./assets/logo.svg";

const Layout = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
    }}
  >
    <div
      style={{ display: "flex", flexDirection: "row", alignItems: "stretch" }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "11px",
          borderRight: "2px solid var(--darkest-color)",
          alignItems: "stretch",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            paddingRight: "11px",
          }}
        >
          <DCO1 />
          <DCO2 />
        </div>
        <div style={{ display: "flex", flexDirection: "row" }}>
          <DCOMod />
        </div>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "stretch",
        }}
      >
        <div style={{ display: "flex", flexDirection: "row" }}>
          <div
            style={{
              paddingLeft: "11px",
              paddingRight: "11px",
              borderRight: "2px solid var(--darkest-color)",
              borderBottom: "2px solid var(--darkest-color)",
            }}
          >
            <Mixer />
          </div>
          <div
            style={{
              paddingLeft: "11px",
              borderRight: "2px solid var(--darkest-color)",
              borderBottom: "2px solid var(--darkest-color)",
            }}
          >
            <VCF />
          </div>
          <div style={{ position: "relative", flexGrow: "1" }}>
            <img
              src={logo}
              alt="logo"
              style={{
                position: "absolute",
                top: 0,
                left: 0,
                width: "100%",
              }}
            />
          </div>
        </div>
        <div style={{ display: "flex", flexDirection: "row" }}>
          <div
            style={{
              paddingTop: "11px",
              paddingRight: "11px",
              paddingLeft: "11px",
              borderRight: "2px solid var(--darkest-color)",
            }}
          >
            <ENV env="1" />
          </div>
          <div
            style={{
              paddingTop: "11px",
              paddingRight: "11px",
              paddingLeft: "11px",
              position: "relative",
              top: "-2px",
              borderTop: "2px solid var(--darkest-color)",
              borderRight: "2px solid var(--darkest-color)",
            }}
          >
            <ENV env="2" />
          </div>
        </div>
      </div>
    </div>
  </div>
);

export default Layout;
