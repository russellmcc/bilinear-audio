import { useUiState } from "@conformal/plugin";
import { useCallback } from "react";
import { z } from "zod";
import type { Preset } from "./preset";
import { useApplyPreset } from "./preset";
import c3pPreset from "./c3p/preset";
import superDimensionPreset from "./super-dimension/preset";
import ce2Preset from "./ce-2/preset";
import jazz120Preset from "./jazz-120/preset";
import { JAZZ_VIBRATO_DEPTH, JAZZ_VIBRATO_RATE } from "./jazz-120/constants";
import ju60Preset from "./ju-60/preset";

const c3pSchema = z.object({
  id: z.literal("c3p"),
});

const superDimensionSchema = z.object({
  id: z.literal("super-dimension"),
});

const ce2Schema = z.object({
  id: z.literal("ce-2"),
});

const jazz120Schema = z.object({
  id: z.literal("jazz-120"),
  chorusMode: z.enum(["vibrato", "chorus"]),
  lastRate: z.number(),
  lastDepth: z.number(),
});

const ju60Schema = z.object({
  id: z.literal("ju-60"),
  buttonMode: z.enum(["I", "II", "III"]),
});

export const modeSchema = z.union([
  c3pSchema,
  superDimensionSchema,
  ce2Schema,
  jazz120Schema,
  ju60Schema,
]);

export const defaultJazz120Mode: Jazz120Mode = {
  chorusMode: "vibrato",
  lastRate: JAZZ_VIBRATO_RATE,
  lastDepth: JAZZ_VIBRATO_DEPTH,
};

export const defaultJu60Mode: Ju60Mode = {
  buttonMode: "I",
};

const modeIds = modeSchema.options.map((option) => option.shape.id.value);

export type Mode = z.infer<typeof modeSchema>;

export type Jazz120Mode = Omit<z.infer<typeof jazz120Schema>, "id">;
export type Ju60Mode = Omit<z.infer<typeof ju60Schema>, "id">;

const makeMode = (id: Mode["id"]): Mode => {
  switch (id) {
    case "c3p":
    case "super-dimension":
    case "ce-2":
      return { id };
    case "jazz-120":
      return {
        id,
        chorusMode: "vibrato",
        lastRate: JAZZ_VIBRATO_RATE,
        lastDepth: JAZZ_VIBRATO_DEPTH,
      };
    case "ju-60":
      return {
        id,
        ...defaultJu60Mode,
      };
  }
};

export const useMode = (): {
  mode: Mode;
  setMode: (mode: Mode) => void;
  jazz120Mode: Jazz120Mode;
  setJazz120Mode: (mode: Jazz120Mode) => void;
  ju60Mode: Ju60Mode;
  setJu60Mode: (mode: Ju60Mode) => void;
} => {
  const { value, set } = useUiState<Mode>();
  const jazz120Mode = value?.id === "jazz-120" ? value : defaultJazz120Mode;
  const ju60Mode = value?.id === "ju-60" ? value : defaultJu60Mode;
  const id = value?.id;
  const setJazz120Mode = useCallback(
    (mode: Jazz120Mode) => {
      if (id !== "jazz-120") {
        return;
      }
      set({
        id,
        ...mode,
      });
    },
    [id, set],
  );
  const setJu60Mode = useCallback(
    (mode: Ju60Mode) => {
      if (id !== "ju-60") {
        return;
      }
      set({
        id,
        ...mode,
      });
    },
    [id, set],
  );
  return {
    mode: value ?? { id: "c3p" },
    setMode: set,
    jazz120Mode,
    setJazz120Mode,
    ju60Mode,
    setJu60Mode,
  };
};

const getPresetForMode = (mode: Mode): Preset => {
  switch (mode.id) {
    case "c3p":
      return c3pPreset;
    case "super-dimension":
      return superDimensionPreset;
    case "ce-2":
      return ce2Preset;
    case "jazz-120":
      return jazz120Preset;
    case "ju-60":
      return ju60Preset;
  }
};

export const useNextMode = () => {
  const { mode, setMode } = useMode();
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
