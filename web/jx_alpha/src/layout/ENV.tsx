import ParamSlider from "../components/ParamSlider";

const ENV = ({ env }: { env: "1" | "2" }) => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      height: "100%",
    }}
  >
    <h1>{`ENV-${env}`}</h1>
    <div
      style={{
        display: "flex",
        flexDirection: "row",
      }}
    >
      <ParamSlider param={`env${env}_t1`} label="T1" scale="labeled" />
      <ParamSlider param={`env${env}_l1`} label="L1" scale="continuation" />
      <ParamSlider param={`env${env}_t2`} label="T2" scale="continuation" />
      <ParamSlider param={`env${env}_l2`} label="L2" scale="continuation" />
      <ParamSlider param={`env${env}_t3`} label="T3" scale="continuation" />
      <ParamSlider param={`env${env}_l3`} label="L3" scale="continuation" />
      <ParamSlider param={`env${env}_t4`} label="T4" scale="continuation" />
      <ParamSlider param={`env${env}_key`} label="KEY" scale="continuation" />
    </div>
  </div>
);

export default ENV;
