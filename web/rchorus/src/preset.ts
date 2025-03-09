import {
  useNumericAtom,
  useStringAtom,
  useBooleanAtom,
} from "@conformal/plugin";
import { typedInfos } from "./mock_infos";
import { useCallback, useRef } from "react";

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

export const useApplyPreset = () => {
  const setters = Object.fromEntries(
    presetParams.map((param) => [
      param,
      atomGetter[presetParamInfos[param].type_specific.t](`params/${param}`)[1],
    ]),
    // Ugh, I really tried to avoid this type assertion, but I think there's no
    // way to get typescript to accept this fromEntries call.
  ) as Setters;

  const settersRef = useRef(setters);
  settersRef.current = setters;

  const applyPreset = useCallback((preset: Preset) => {
    const applyParam = <K extends PresetParam>(
      value: Preset[K],
      setter: Setters[K],
    ) => {
      setter(value);
    };

    presetParams.forEach((key) => {
      applyParam(preset[key], settersRef.current[key]);
    });
  }, []);

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
