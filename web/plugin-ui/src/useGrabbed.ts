import { useCallback, useState } from "react";

export const useGrabbed = ({
  grab,
  release,
}: {
  grab: () => void;
  release: () => void;
}) => {
  const [grabbed, setGrabbed] = useState(false);

  const onGrabOrRelease = useCallback(
    (grabbed: boolean) => {
      setGrabbed(grabbed);
      if (grabbed) {
        grab();
      } else {
        release();
      }
    },
    [grab, release, setGrabbed],
  );

  return { grabbed, onGrabOrRelease };
};
