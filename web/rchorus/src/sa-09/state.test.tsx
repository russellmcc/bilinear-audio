import { describe, expect, test } from "bun:test";
import { act, renderHook } from "@testing-library/react";
import { useEnumParam, useNumericParam } from "@conformal/plugin";
import { RootProviders } from "../Root";
import type { Mode } from "../mode";
import { defaultSa09Mode, useMode, useNextMode } from "../mode";
import {
  SA_09_CHORUS_DEPTH,
  SA_09_CHORUS_RATE,
  SA_09_VIBRATO_DEPTH,
} from "./preset";
import { useSa09State } from "./state";

const useSa09Harness = () => {
  const mode = useMode();
  const rate = useNumericParam("rate");
  const depth = useNumericParam("depth");
  const dryHighpassCutoff = useEnumParam("dry_highpass_cutoff");
  const routing = useEnumParam("routing");

  const sa09 = useSa09State({
    mode: mode.sa09Mode,
    setMode: mode.setSa09Mode,
  });
  return { mode, rate, depth, dryHighpassCutoff, routing, sa09 };
};

const useSa09CycleHarness = () => {
  const mode = useMode();
  const nextMode = useNextMode();
  const rate = useNumericParam("rate");
  const depth = useNumericParam("depth");
  const dryHighpassCutoff = useEnumParam("dry_highpass_cutoff");
  const routing = useEnumParam("routing");

  return { mode, nextMode, rate, depth, dryHighpassCutoff, routing };
};

const getSa09Mode = (mode: Mode | undefined) => {
  if (mode?.id !== "sa-09") {
    throw new Error("Expected SA-09 mode");
  }
  return mode;
};

describe("useSa09State", () => {
  test("vibrato rate is remembered while chorus mode fixes dsp params", () => {
    const { result } = renderHook(useSa09Harness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({
        id: "sa-09",
        ...defaultSa09Mode,
        chorusMode: "vibrato",
      });
    });
    act(() => {
      result.current.sa09.vibratoRate.set(7.25);
    });

    const vibratoMode = getSa09Mode(result.current.mode.mode);
    expect(result.current.rate.value).toBe(7.25);
    expect(result.current.depth.value).toBe(SA_09_VIBRATO_DEPTH);
    expect(vibratoMode.lastRate).toBe(7.25);

    act(() => {
      result.current.sa09.setChorusMode("chorus");
    });

    const chorusMode = getSa09Mode(result.current.mode.mode);
    expect(chorusMode.chorusMode).toBe("chorus");
    expect(chorusMode.lastRate).toBe(7.25);
    expect(result.current.rate.value).toBe(SA_09_CHORUS_RATE);
    expect(result.current.depth.value).toBe(SA_09_CHORUS_DEPTH);

    act(() => {
      result.current.sa09.vibratoRate.set(5);
    });

    expect(result.current.rate.value).toBe(SA_09_CHORUS_RATE);

    act(() => {
      result.current.sa09.setChorusMode("vibrato");
    });

    const restoredMode = getSa09Mode(result.current.mode.mode);
    expect(restoredMode.chorusMode).toBe("vibrato");
    expect(result.current.rate.value).toBe(7.25);
    expect(result.current.depth.value).toBe(SA_09_VIBRATO_DEPTH);
  });

  test("routing modes set routing and dry highpass params", () => {
    const { result } = renderHook(useSa09Harness, {
      wrapper: RootProviders,
    });

    act(() => {
      result.current.mode.setMode({ id: "sa-09", ...defaultSa09Mode });
    });
    act(() => {
      result.current.sa09.setRoutingMode("II");
    });

    const modeII = getSa09Mode(result.current.mode.mode);
    expect(modeII.routingMode).toBe("II");
    expect(result.current.routing.value).toBe("Synth");
    expect(result.current.dryHighpassCutoff.value).toBe("High");

    act(() => {
      result.current.sa09.setRoutingMode("I");
    });

    const modeI = getSa09Mode(result.current.mode.mode);
    expect(modeI.routingMode).toBe("I");
    expect(result.current.routing.value).toBe("Pedal");
    expect(result.current.dryHighpassCutoff.value).toBe("Low");
  });

  test("mode carousel enters SA-09 in chorus mode I", () => {
    const { result } = renderHook(useSa09CycleHarness, {
      wrapper: RootProviders,
    });

    for (let i = 0; i < 11; i += 1) {
      act(() => {
        result.current.nextMode();
      });
    }

    const mode = getSa09Mode(result.current.mode.mode);
    expect(mode.chorusMode).toBe("chorus");
    expect(mode.routingMode).toBe("I");
    expect(result.current.rate.value).toBe(SA_09_CHORUS_RATE);
    expect(result.current.depth.value).toBe(SA_09_CHORUS_DEPTH);
    expect(result.current.routing.value).toBe("Pedal");
    expect(result.current.dryHighpassCutoff.value).toBe("Low");
  });
});
