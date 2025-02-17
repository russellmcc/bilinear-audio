import { useMemo } from "react";
import { Props } from ".";
import Display from "./Display.tsx";
import { Knob as KnobKit } from "music-ui/kit";

const PRIMARY_KNOB_SIZE = 61;
const SECONDARY_KNOB_SIZE = 31;
const PRIMARY_RADIUS_RATIO = 18 / 30.5;
const SECONDARY_RADIUS_RATIO = 15 / 30.5;

export const useDisplay = ({
  style = "primary",
}: {
  style?: Props["style"];
}) => {
  const display = useMemo(() => {
    const StyledDisplay = (props: KnobKit.DisplayProps) => (
      <Display
        size={style === "primary" ? PRIMARY_KNOB_SIZE : SECONDARY_KNOB_SIZE}
        innerRadiusRatio={
          style === "primary" ? PRIMARY_RADIUS_RATIO : SECONDARY_RADIUS_RATIO
        }
        {...props}
      />
    );
    return StyledDisplay;
  }, [style]);
  return display;
};
