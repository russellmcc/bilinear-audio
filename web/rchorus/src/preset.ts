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

const atomSetters = {
  numeric: useNumericAtom,
  enum: useStringAtom,
  switch: useBooleanAtom,
} as const;
type ParamType = keyof typeof atomSetters;

type SetterForParamType<T extends ParamType> = ReturnType<
  (typeof atomSetters)[T]
>[1];

const useSelectSetter = <T extends ParamType>(
  t: T,
  key: string,
): SetterForParamType<T> => atomSetters[t](key)[1];

type ParamTypeOf<K extends PresetParam> =
  (typeof presetParamInfos)[K]["type_specific"]["t"];

type ValueOf<K extends PresetParam> = Parameters<
  SetterForParamType<ParamTypeOf<K>>
>[0];

export type Preset = {
  [Parameter in PresetParam]: ValueOf<Parameter>;
};

type Setters = {
  [Param in PresetParam]: SetterForParamType<ParamTypeOf<Param>>;
};

const useSetter = <Param extends PresetParam>(param: Param) => {
  const t: ParamTypeOf<Param> = presetParamInfos[param].type_specific.t;
  return useSelectSetter(t, `params/${param}`);
};

const applyMapped = <M extends Record<string, unknown>, K extends keyof M>(
  keys: readonly K[],
  map: M,
  setters: { [P in K]: (v: M[P]) => void },
) => {
  keys.forEach((k) => {
    setters[k](map[k]);
  });
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
      applyMapped(presetParams, preset, setters);
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
