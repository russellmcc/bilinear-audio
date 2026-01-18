import { useNumericParam } from "@conformal/plugin";
import { Slider } from "./components/Slider";

const Layout = () => {
  const { value, set, grab, release } = useNumericParam("level");
  return (
    <>
      <Slider
        value={value}
        label="FOLLOW"
        onValue={set}
        grab={grab}
        release={release}
        scale="labeled"
      />
      <Slider
        value={value}
        label="FOLLOW"
        onValue={set}
        grab={grab}
        release={release}
        scale="continuation"
      />
    </>
  );
};

export default Layout;
