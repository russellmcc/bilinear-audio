export const formatDynMode = (value: string) => {
  switch (value) {
    case "Velocity":
      return "VEL";
    case "Touch":
      return "Z";
    case "Timbre":
      return "Y";
    case "Velocity+Touch":
      return "V+Z";
    case "Velocity+Timbre":
      return "V+Y";
    default:
      return value;
  }
};
