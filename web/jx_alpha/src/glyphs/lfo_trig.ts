export const formatLfoTrig = (value: string) => {
  if (value.toLowerCase() === "auto") return "AUTO";
  return "WHL";
};

export default formatLfoTrig;
