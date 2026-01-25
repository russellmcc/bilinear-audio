import Adsr, { AdsrInverted } from "./Adsr";

export type Props = {
  value: string;
};

export const Dynamic = () => <span>D</span>;

export const EnvSource = ({ value }: Props) => {
  // Special case: "dynamic" should say DYN
  if (value.toLowerCase() === "dynamic") {
    return <span>DYN</span>;
  }
  const isDynamic = value.toLowerCase().includes("dynamic");
  const isInverse = value.toLowerCase().includes("inverse");

  return (
    <div
      style={{
        display: "inline-flex",
        alignItems: "center",
        justifyContent: "flex-end",
        height: "100%",
        verticalAlign: "middle",
      }}
    >
      {isDynamic && <Dynamic />}
      {isInverse ? <AdsrInverted /> : <Adsr />}
    </div>
  );
};

export default EnvSource;
