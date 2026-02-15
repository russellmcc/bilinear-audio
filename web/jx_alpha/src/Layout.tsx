import DCO1 from "./layout/DCO1";
import DCO2 from "./layout/DCO2";
import DCOMod from "./layout/DCOMod";
import ENV from "./layout/ENV";
import Mixer from "./layout/Mixer";
import VCF from "./layout/VCF";
import logo from "./assets/logo.svg";
import LFO from "./layout/LFO";

const Layout = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
    }}
  >
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        alignItems: "stretch",
        paddingLeft: "22px",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "9px",
          alignItems: "flex-start",
          justifyContent: "space-between",
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
        <div
          style={{
            display: "flex",
            flexDirection: "row",
          }}
        >
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
              paddingTop: "11px",
              paddingLeft: "11px",
              paddingRight: "11px",
              width: "141px",
              borderLeft: "2px solid var(--darkest-color)",
              borderRight: "2px solid var(--darkest-color)",
            }}
          >
            <Mixer />
          </div>
          <div
            style={{
              paddingTop: "11px",
              paddingLeft: "11px",
            }}
          >
            <VCF />
          </div>
          <div
            style={{
              position: "relative",
              display: "flex",
              flexDirection: "column",
            }}
          >
            <div
              style={{
                position: "relative",
                display: "flex",
                flexDirection: "column",
              }}
            >
              <div
                style={{
                  position: "relative",
                  marginLeft: "-105px",
                  paddingTop: "14px",
                  paddingLeft: "11px",
                  display: "flex",
                  flexDirection: "column",
                  borderLeft: "2px solid var(--darkest-color)",
                }}
              >
                <h1 className="title">SYNTHESIZER PROGRAMMER</h1>
                <img
                  src={logo}
                  alt="logo"
                  style={{
                    width: "205px",
                  }}
                />
              </div>
              <LFO />
            </div>
          </div>
        </div>
        <div
          style={{
            marginLeft: "-130px",
            display: "flex",
            flexDirection: "row",
            alignItems: "flex-start",
            borderLeft: "2px solid var(--darkest-color)",
          }}
        >
          <div
            style={{
              paddingTop: "11px",
              paddingRight: "11px",
              paddingLeft: "11px",
              borderTop: "2px solid var(--darkest-color)",
              borderRight: "2px solid var(--darkest-color)",
              paddingBottom: "11px",
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
              borderTop: "2px solid var(--darkest-color)",
              display: "flex",
              flexDirection: "row",
              justifyContent: "center",
              width: "325px",
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
