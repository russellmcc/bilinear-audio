import { useCallback } from "react";

export type Props = {
  grab?: () => void;
  release?: () => void;
};

export const useOnGrabOrRelease = ({ grab, release }: Props) =>
  useCallback(
    (grabbed: boolean) => {
      if (grabbed) {
        grab?.();
      } else {
        release?.();
      }
    },
    [grab, release],
  );

export default useOnGrabOrRelease;
