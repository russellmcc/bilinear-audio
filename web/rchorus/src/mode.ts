import { useUiStateAtom } from "@conformal/plugin";
import { useAtomValue, useSetAtom } from "jotai";
import { useCallback } from "react";
import { z } from "zod";
import { Preset, useApplyPreset } from "./preset";
import c3pPreset from "./c3p/preset";
import superDimensionPreset from "./super-dimension/preset";

const c3pSchema = z.object({
  id: z.literal("c3p"),
});

const superDimensionSchema = z.object({
  id: z.literal("super-dimension"),
});

export const modeSchema = z.union([c3pSchema, superDimensionSchema]);

const modeIds = modeSchema.options.map((option) => option.shape.id.value);

export type Mode = z.infer<typeof modeSchema>;

const makeMode = (id: Mode["id"]): Mode => ({ id });

export const useMode = (): Mode => {
  const mode = useAtomValue(useUiStateAtom<Mode>());
  return mode ?? { id: "c3p" };
};

const getPresetForMode = (mode: Mode): Preset => {
  switch (mode.id) {
    case "c3p":
      return c3pPreset;
    case "super-dimension":
      return superDimensionPreset;
  }
};

export const useNextMode = () => {
  const mode = useMode();
  const setMode = useSetAtom(useUiStateAtom<Mode>());
  const applyPreset = useApplyPreset();
  const nextMode = useCallback(() => {
    const currentIndex = modeIds.indexOf(mode.id);
    if (currentIndex === -1) {
      throw new Error("Internal error: mode not found");
    }
    const nextIndex = (currentIndex + 1) % modeIds.length;
    const nextModeId = modeIds[nextIndex]!;
    const nextMode = makeMode(nextModeId);
    applyPreset(getPresetForMode(nextMode));
    setMode(nextMode);
  }, [mode.id, setMode, applyPreset]);
  return nextMode;
};
