import { useMode } from "./mode";
import C3PLayout from "./c3p/layout";
import SuperDimensionLayout from "./super-dimension/layout";
import Ce2Layout from "./ce-2/layout";
import Jazz120Layout from "./jazz-120/layout";
import Ju60Layout from "./ju-60/layout";
const Router = () => {
  const { mode, jazz120Mode, setJazz120Mode, ju60Mode, setJu60Mode } =
    useMode();
  switch (mode.id) {
    case "c3p":
      return <C3PLayout />;
    case "super-dimension":
      return <SuperDimensionLayout />;
    case "ce-2":
      return <Ce2Layout />;
    case "jazz-120":
      return <Jazz120Layout mode={jazz120Mode} setMode={setJazz120Mode} />;
    case "ju-60":
      return <Ju60Layout mode={ju60Mode} setMode={setJu60Mode} />;
  }
};

export default Router;
