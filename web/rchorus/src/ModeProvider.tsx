import { codecFromZod } from "@conformal/plugin";
import { useMemo } from "react";
import { modeSchema } from "./mode";
import { UiStateProvider as UiStateProviderInternal } from "@conformal/plugin";

export const ModeProvider = ({ children }: { children: React.ReactNode }) => {
  const codec = useMemo(() => codecFromZod(modeSchema), []);
  return (
    <UiStateProviderInternal codec={codec}>{children}</UiStateProviderInternal>
  );
};

export default ModeProvider;
