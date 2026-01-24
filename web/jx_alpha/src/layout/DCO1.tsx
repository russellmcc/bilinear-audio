import ParamEnumSlider from "../components/ParamEnumSlider";
import Shape from "../glyphs/Shape";

export const DCO1 = () => (
  <div>
    <h1>DCO-1</h1>
    <div style={{ display: "flex", flexDirection: "row" }}>
      <ParamEnumSlider param="dco1_range" label="RANGE" order="reversed" />
      <ParamEnumSlider param="dco1_shape" label="SHAPE" CustomGlyph={Shape} />
    </div>
  </div>
);

export default DCO1;
