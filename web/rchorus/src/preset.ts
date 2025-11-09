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

const atomGetter = {
  numeric: useNumericAtom,
  enum: useStringAtom,
  switch: useBooleanAtom,
} as const;

export type Preset = {
  [Parameter in PresetParam]: Parameters<
    ReturnType<
      (typeof atomGetter)[(typeof presetParamInfos)[Parameter]["type_specific"]["t"]]
    >[1]
  >[0];
};

type Setters = {
  [K in PresetParam]: (value: Preset[K]) => void;
};

const useSetter = <P extends PresetParam>(param: P) => {
  const type: (typeof presetParamInfos)[P]["type_specific"]["t"] =
    presetParamInfos[param].type_specific.t;
  const getter = atomGetter[type] as (typeof atomGetter)[typeof type];
  const ret = getter(`params/${param}`)[1] as ReturnType<typeof getter>[1];
  return ret;
};

export const useApplyPreset = () => {
  const rate = useSetter("rate");
  const depth = useSetter("depth");
  const mix = useSetter("mix");
  const highpass_cutoff = useSetter("highpass_cutoff");
  const routing = useSetter("routing");
  const setters: Setters = useMemo(
    () => ({
      rate,
      depth,
      mix,
      highpass_cutoff,
      routing,
    }),
    [depth, highpass_cutoff, mix, rate, routing],
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
