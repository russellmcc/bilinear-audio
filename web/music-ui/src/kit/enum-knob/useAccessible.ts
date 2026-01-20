import { useCallback, useMemo, useState } from "react";

export type UseAccessibleProps = {
  value?: string;
  values: string[];
  onValue?: (value: string) => void;
  displayFormatter?: (value: string) => string;
  label: string;
  accessibilityLabel?: string;
};

export const useAccessible = ({
  value,
  values,
  onValue,
  displayFormatter,
  label,
  accessibilityLabel,
}: UseAccessibleProps) => {
  const [interacted, setInteracted] = useState(false);
  const valueNumber = useMemo(
    () => (value ? values.indexOf(value) : undefined) ?? 0,
    [value, values],
  );
  const valueCount = useMemo(() => values.length, [values]);
  const valueLabel = useMemo(
    () => (value ? (displayFormatter?.(value) ?? value) : undefined),
    [value, displayFormatter],
  );
  const labelText = useMemo(
    () => accessibilityLabel ?? label,
    [accessibilityLabel, label],
  );
  const onBlur = useCallback(() => {
    setInteracted(false);
  }, []);
  const onKeyDown = useCallback(
    (event: React.KeyboardEvent) => {
      setInteracted(true);
      event.preventDefault();
      event.stopPropagation();
      switch (event.code) {
        case "ArrowRight":
        case "ArrowUp":
          onValue?.(values[(valueNumber + 1) % valueCount]!);
          break;
        case "ArrowLeft":
        case "ArrowDown":
          onValue?.(values[(valueNumber - 1 + valueCount) % valueCount]!);
          break;
        case "End":
          onValue?.(values[valueCount - 1]!);
          break;
        case "Home":
          onValue?.(values[0]!);
          break;
      }
    },
    [onValue, valueCount, valueNumber, values],
  );
  return {
    props: {
      role: "spinbutton",
      "aria-label": labelText,
      "aria-valuemin": 0,
      "aria-valuemax": valueCount ? valueCount - 1 : 0,
      "aria-valuenow": valueNumber,
      "aria-valuetext": valueLabel,
      tabIndex: 0,
      onBlur,
      onKeyDown,
    },
    interacted,
  };
};

export default useAccessible;
