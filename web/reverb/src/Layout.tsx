import ControlLayout from "./control-layout";

const Layout = () => (
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      justifyContent: "space-between",
      position: "absolute",
      top: "0",
      left: "0",
      right: "0",
      bottom: "0",
    }}
  >
    <h1>Fluffyverb</h1>
    <ControlLayout />
  </div>
);

export default Layout;
