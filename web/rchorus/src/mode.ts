import { useUiStateAtom } from "@conformal/plugin";
import { useAtomValue, useSetAtom } from "jotai";
import { useCallback } from "react";
import { z } from "zod";
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

const modeSetSideEffect = (mode: Mode) => {
  switch (mode.id) {
    case "c3p":
      break;
    case "super-dimension":
      break;
    default:
      mode satisfies never;
  }
};

export const useNextMode = () => {
  const mode = useMode();
  const setMode = useSetAtom(useUiStateAtom<Mode>());
  const nextMode = useCallback(() => {
    const currentIndex = modeIds.indexOf(mode.id);
    if (currentIndex === -1) {
      throw new Error("Internal error: mode not found");
    }
    const nextIndex = (currentIndex + 1) % modeIds.length;
    const nextModeId = modeIds[nextIndex]!;
    const nextMode = makeMode(nextModeId);
    modeSetSideEffect(nextMode);
    setMode(nextMode);
  }, [mode.id, setMode]);
  return nextMode;
};
