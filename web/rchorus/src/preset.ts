import {
  useNumericAtom,
  useStringAtom,
  useBooleanAtom,
} from "@conformal/plugin";
import { typedInfos } from "./mock_infos";
import { useCallback, useMemo } from "react";

const presetParams = [
  "rate",
  "depth",
  "mix",
  "highpass_cutoff",
  "routing",
] as const;

type PresetParam = (typeof presetParams)[number];

const presetParamInfos: {
  [K in PresetParam]: (typeof typedInfos)[K];
} = {
  rate: typedInfos.rate,
  depth: typedInfos.depth,
  mix: typedInfos.mix,
  highpass_cutoff: typedInfos.highpass_cutoff,
  routing: typedInfos.routing,
} as const;

export type Preset = {
  [Parameter in PresetParam]: (typeof presetParamInfos)[Parameter]["type_specific"]["default"];
};

const atomGetter = {
  numeric: useNumericAtom,
  enum: useStringAtom,
  switch: useBooleanAtom,
} as const;

type Setters = {
  [K in PresetParam]: (value: Preset[K]) => void;
};

export const useApplyPreset = () => {
  const setters: Setters = useMemo(
    () => ({
      rate: atomGetter[presetParamInfos.rate.type_specific.t]("rate")[1],
      depth: atomGetter[presetParamInfos.depth.type_specific.t]("depth")[1],
      mix: atomGetter[presetParamInfos.mix.type_specific.t]("mix")[1],
      highpass_cutoff:
        atomGetter[presetParamInfos.highpass_cutoff.type_specific.t](
          "highpass_cutoff",
        )[1],
      routing:
        atomGetter[presetParamInfos.routing.type_specific.t]("routing")[1],
    }),
    [],
  );

  const applyPreset = useCallback(
    (preset: Preset) => {
      const applyParam = <K extends PresetParam>(
        value: Preset[K],
        setter: Setters[K],
      ) => {
        setter(value);
      };

      presetParams.forEach((key) => {
        applyParam(preset[key], setters[key]);
      });
    },
    [setters],
  );

  return applyPreset;
};

export const applyPreset = (_preset: Preset) => {
  presetParams.forEach((key) => {
    const paramInfo = presetParamInfos[key];
    switch (paramInfo.type_specific.t) {
      case "numeric":
        break;
      case "enum":
        break;
      default:
        paramInfo.type_specific satisfies never;
    }
  });
};
