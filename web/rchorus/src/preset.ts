import {
  useNumericParam,
  useEnumParam,
  useSwitchParam,
} from "@conformal/plugin";
import { typedInfos } from "./mock_infos";
import { useCallback, useMemo } from "react";

type PresetParamInfos = Pick<
  typeof typedInfos,
  | "rate"
  | "rate_2"
  | "rate_3"
  | "rate_4"
  | "depth"
  | "ens_depth"
  | "mix"
  | "delay_scale"
  | "highpass_cutoff"
  | "dry_highpass_cutoff"
  | "routing"
  | "dimension_same_side_pad"
>;

type PresetParam = keyof PresetParamInfos;

const paramHooks = {
  numeric: useNumericParam,
  enum: useEnumParam,
  switch: useSwitchParam,
} as const;
type ParamType = keyof typeof paramHooks;

type SetterForParamType<T extends ParamType> = ReturnType<
  (typeof paramHooks)[T]
>["set"];

const useSelectSetter = <T extends ParamType>(
  t: T,
  key: string,
): SetterForParamType<T> => paramHooks[t](key).set;

type ParamTypeOf<K extends PresetParam> =
  PresetParamInfos[K]["type_specific"]["t"];

type ValueOf<K extends PresetParam> = Parameters<
  SetterForParamType<ParamTypeOf<K>>
>[0];

export type Preset = {
  rate: ValueOf<"rate">;
  depth: ValueOf<"depth">;
  mix: ValueOf<"mix">;
  highpass_cutoff: ValueOf<"highpass_cutoff">;
  dry_highpass_cutoff: ValueOf<"dry_highpass_cutoff">;
  routing: ValueOf<"routing">;
  delay_scale: ValueOf<"delay_scale">;
  rate_2?: ValueOf<"rate_2">;
  rate_3?: ValueOf<"rate_3">;
  rate_4?: ValueOf<"rate_4">;
  ens_depth?: ValueOf<"ens_depth">;
  dimension_same_side_pad?: ValueOf<"dimension_same_side_pad">;
};

export const useApplyPreset = () => {
  const rate = useSelectSetter("numeric", "rate");
  const rate_2 = useSelectSetter("numeric", "rate_2");
  const rate_3 = useSelectSetter("numeric", "rate_3");
  const rate_4 = useSelectSetter("numeric", "rate_4");
  const depth = useSelectSetter("numeric", "depth");
  const ens_depth = useSelectSetter("numeric", "ens_depth");
  const mix = useSelectSetter("numeric", "mix");
  const delay_scale = useSelectSetter("numeric", "delay_scale");
  const highpass_cutoff = useSelectSetter("enum", "highpass_cutoff");
  const dry_highpass_cutoff = useSelectSetter("enum", "dry_highpass_cutoff");
  const routing = useSelectSetter("enum", "routing");
  const dimension_same_side_pad = useSelectSetter(
    "numeric",
    "dimension_same_side_pad",
  );
  const setters = useMemo(
    () => ({
      rate,
      rate_2,
      rate_3,
      rate_4,
      depth,
      ens_depth,
      mix,
      delay_scale,
      highpass_cutoff,
      dry_highpass_cutoff,
      routing,
      dimension_same_side_pad,
    }),
    [
      depth,
      delay_scale,
      dimension_same_side_pad,
      dry_highpass_cutoff,
      ens_depth,
      highpass_cutoff,
      mix,
      rate,
      rate_2,
      rate_3,
      rate_4,
      routing,
    ],
  );

  const applyPreset = useCallback(
    (preset: Preset) => {
      setters.rate(preset.rate);
      setters.depth(preset.depth);
      setters.mix(preset.mix);
      setters.highpass_cutoff(preset.highpass_cutoff);
      setters.dry_highpass_cutoff(preset.dry_highpass_cutoff);
      setters.routing(preset.routing);
      setters.delay_scale(preset.delay_scale);
      if (preset.rate_2 !== undefined) {
        setters.rate_2(preset.rate_2);
      }
      if (preset.rate_3 !== undefined) {
        setters.rate_3(preset.rate_3);
      }
      if (preset.rate_4 !== undefined) {
        setters.rate_4(preset.rate_4);
      }
      if (preset.ens_depth !== undefined) {
        setters.ens_depth(preset.ens_depth);
      }
      if (preset.dimension_same_side_pad !== undefined) {
        setters.dimension_same_side_pad(preset.dimension_same_side_pad);
      }
    },
    [setters],
  );

  return applyPreset;
};
