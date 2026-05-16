import { useMode } from "./mode";
import C3PLayout from "./c3p/layout";
import SuperDimensionLayout from "./super-dimension/layout";
import Ce2Layout from "./ce-2/layout";
import Jazz120Layout from "./jazz-120/layout";
const Router = () => {
  const { mode, jazz120Mode, setJazz120Mode } = useMode();
  switch (mode.id) {
    case "c3p":
      return <C3PLayout />;
    case "super-dimension":
      return <SuperDimensionLayout />;
    case "ce-2":
      return <Ce2Layout />;
    case "jazz-120":
      return <Jazz120Layout mode={jazz120Mode} setMode={setJazz120Mode} />;
  }
};

export default Router;
