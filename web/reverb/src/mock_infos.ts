import { Info } from "@conformal/plugin";

const infos = new Map<string, Info>(
  Object.entries({
    bypass: {
      title: "Bypass",
      type_specific: {
        t: "switch",
        default: false,
      },
    },
    mix: {
      title: "Mix",
      type_specific: {
        t: "numeric",
        default: 50,
        valid_range: [0, 100],
        units: "%",
      },
    },
    brightness: {
      title: "Brightness",
      type_specific: {
        t: "numeric",
        default: 100,
        valid_range: [0, 100],
        units: "%",
      },
    },
    tone: {
      title: "Tone",
      type_specific: {
        t: "numeric",
        default: 100,
        valid_range: [0, 100],
        units: "%",
      },
    },
    time: {
      title: "Time",
      type_specific: {
        t: "numeric",
        default: 1.2,
        valid_range: [0.7, 3.1],
        units: "s",
      },
    },
    early_reflections: {
      title: "Early Reflections Character",
      type_specific: {
        t: "numeric",
        default: 50,
        valid_range: [0, 100],
        units: "%",
      },
    },
    density: {
      title: "Density",
      type_specific: {
        t: "numeric",
        default: 100,
        valid_range: [0, 100],
        units: "%",
      },
    },
  }),
);

export default infos;
