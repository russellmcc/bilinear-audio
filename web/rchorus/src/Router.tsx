import { useMode } from "./mode";
import C3PLayout from "./c3p/layout";
import SuperDimensionLayout from "./super-dimension/layout";
import Ce2Layout from "./ce-2/layout";
const Router = () => {
  const mode = useMode();
  switch (mode.id) {
    case "c3p":
      return <C3PLayout />;
    case "super-dimension":
      return <SuperDimensionLayout />;
    case "ce-2":
      return <Ce2Layout />;
  }
};

export default Router;
