import DCO1 from "./layout/DCO1";
import DCO2 from "./layout/DCO2";
import DCOMod from "./layout/DCOMod";
import ENV from "./layout/ENV";
import LFO from "./layout/LFO";
import Mixer from "./layout/Mixer";
import VCA from "./layout/VCA";
import VCF from "./layout/VCF";

const Layout = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
    }}
  >
    <div style={{ display: "flex", flexDirection: "row" }}>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "11px",
          borderRight: "2px solid var(--darkest-color)",
          borderBottom: "2px solid var(--darkest-color)",
          alignItems: "stretch",
        }}
      >
        <div
          style={{ display: "flex", flexDirection: "row", paddingRight: "5px" }}
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
          paddingRight: "11px",
          borderRight: "2px solid var(--darkest-color)",
          borderBottom: "2px solid var(--darkest-color)",
        }}
      >
        <VCF />
      </div>
      <div
        style={{
          paddingLeft: "11px",
          paddingRight: "11px",
          borderRight: "2px solid var(--darkest-color)",
          borderBottom: "2px solid var(--darkest-color)",
        }}
      >
        <div style={{ display: "flex", flexDirection: "column", gap: "11px" }}>
          <VCA />
          <LFO />
          <ENV env="1" />
          <ENV env="2" />
        </div>
      </div>
    </div>
  </div>
);

export default Layout;
