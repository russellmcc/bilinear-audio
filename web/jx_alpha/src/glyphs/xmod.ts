export const formatXmod = (value: string) => {
  if (value.toLowerCase() === "off") return "OFF";
  const firstParts = value
    .split("+")
    .map((v) => (v.length > 0 ? v[0]!.toUpperCase() : ""));
  return firstParts.join("+");
};
