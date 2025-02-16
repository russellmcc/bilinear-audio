import { Knob as KnobKit } from "music-ui/kit";

const Label = ({ label, valueLabel, hover, grabbed }: KnobKit.LabelProps) => (
  <div className={"relative mt-[-5px] font-sans"}>
    <div
      className="text-border text-center transition-opacity duration-500 ease-in"
      style={{ opacity: hover || grabbed ? "0%" : "100%" }}
    >
      {label}
    </div>
    <div
      className="text-pop absolute inset-0 text-center blur-[1px] transition-opacity duration-1000 ease-in"
      style={{ opacity: hover || grabbed ? "100%" : "0%" }}
    >
      {valueLabel}
    </div>
    <div
      className="text-pop absolute inset-0 text-center transition-opacity duration-1000 ease-in"
      style={{ opacity: hover || grabbed ? "100%" : "0%" }}
    >
      {valueLabel}
    </div>
  </div>
);

export default Label;
