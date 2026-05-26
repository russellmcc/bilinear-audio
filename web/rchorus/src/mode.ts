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
import rs79Preset from "./rs-79/preset";
import { RS_79_DEFAULT_ENSEMBLE_MODE } from "./rs-79/constants";
import stringPreset from "./string/preset";
import { STRING_DEFAULT_ENSEMBLE_MODE } from "./string/constants";
import rs101Preset from "./rs-101/preset";
import ensemblePlusPreset from "./ensemble-plus/preset";
import {
  ENSEMBLE_PLUS_DEFAULT_MODE,
  HUMAN_VOICE_DEFAULT_ENS_DEPTH,
  HUMAN_VOICE_DEFAULT_RATE,
} from "./ensemble-plus/constants";

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

const rs79Schema = z.object({
  id: z.literal("rs-79"),
  ensembleMode: z.enum(["I", "II"]),
});

const stringSchema = z.object({
  id: z.literal("string"),
  ensembleMode: z.enum(["I", "II"]),
});

const rs101Schema = z.object({
  id: z.literal("rs-101"),
});

const ensemblePlusSchema = z.object({
  id: z.literal("ensemble-plus"),
  voiceMode: z.enum(["humanVoice", "strings"]),
  lastRate: z.number(),
  lastEnsDepth: z.number(),
});

export const modeSchema = z.union([
  c3pSchema,
  superDimensionSchema,
  ce2Schema,
  jazz120Schema,
  ju60Schema,
  rs79Schema,
  stringSchema,
  rs101Schema,
  ensemblePlusSchema,
]);

export const defaultJazz120Mode: Jazz120Mode = {
  chorusMode: "vibrato",
  lastRate: JAZZ_VIBRATO_RATE,
  lastDepth: JAZZ_VIBRATO_DEPTH,
};

export const defaultJu60Mode: Ju60Mode = {
  buttonMode: "I",
};

export const defaultRs79Mode: Rs79Mode = {
  ensembleMode: RS_79_DEFAULT_ENSEMBLE_MODE,
};

export const defaultStringMode: StringMode = {
  ensembleMode: STRING_DEFAULT_ENSEMBLE_MODE,
};

export const defaultEnsemblePlusMode: EnsemblePlusMode = {
  voiceMode: ENSEMBLE_PLUS_DEFAULT_MODE,
  lastRate: HUMAN_VOICE_DEFAULT_RATE,
  lastEnsDepth: HUMAN_VOICE_DEFAULT_ENS_DEPTH,
};

const modeIds = modeSchema.options.map((option) => option.shape.id.value);

export type Mode = z.infer<typeof modeSchema>;

export type Jazz120Mode = Omit<z.infer<typeof jazz120Schema>, "id">;
export type Ju60Mode = Omit<z.infer<typeof ju60Schema>, "id">;
export type Rs79Mode = Omit<z.infer<typeof rs79Schema>, "id">;
export type StringMode = Omit<z.infer<typeof stringSchema>, "id">;
export type EnsemblePlusMode = Omit<z.infer<typeof ensemblePlusSchema>, "id">;

const makeMode = (id: Mode["id"]): Mode => {
  switch (id) {
    case "c3p":
    case "super-dimension":
    case "ce-2":
    case "rs-101":
      return { id };
    case "ensemble-plus":
      return {
        id,
        ...defaultEnsemblePlusMode,
      };
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
    case "rs-79":
      return {
        id,
        ...defaultRs79Mode,
      };
    case "string":
      return {
        id,
        ...defaultStringMode,
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
  rs79Mode: Rs79Mode;
  setRs79Mode: (mode: Rs79Mode) => void;
  stringMode: StringMode;
  setStringMode: (mode: StringMode) => void;
  ensemblePlusMode: EnsemblePlusMode;
  setEnsemblePlusMode: (mode: EnsemblePlusMode) => void;
} => {
  const { value, set } = useUiState<Mode>();
  const jazz120Mode = value?.id === "jazz-120" ? value : defaultJazz120Mode;
  const ju60Mode = value?.id === "ju-60" ? value : defaultJu60Mode;
  const rs79Mode = value?.id === "rs-79" ? value : defaultRs79Mode;
  const stringMode = value?.id === "string" ? value : defaultStringMode;
  const ensemblePlusMode =
    value?.id === "ensemble-plus" ? value : defaultEnsemblePlusMode;
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
  const setRs79Mode = useCallback(
    (mode: Rs79Mode) => {
      if (id !== "rs-79") {
        return;
      }
      set({
        id,
        ...mode,
      });
    },
    [id, set],
  );
  const setStringMode = useCallback(
    (mode: StringMode) => {
      if (id !== "string") {
        return;
      }
      set({
        id,
        ...mode,
      });
    },
    [id, set],
  );
  const setEnsemblePlusMode = useCallback(
    (mode: EnsemblePlusMode) => {
      if (id !== "ensemble-plus") {
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
    rs79Mode,
    setRs79Mode,
    stringMode,
    setStringMode,
    ensemblePlusMode,
    setEnsemblePlusMode,
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
    case "rs-79":
      return rs79Preset;
    case "string":
      return stringPreset;
    case "rs-101":
      return rs101Preset;
    case "ensemble-plus":
      return ensemblePlusPreset;
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
