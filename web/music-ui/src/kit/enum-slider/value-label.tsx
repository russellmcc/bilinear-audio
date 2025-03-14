import * as React from "react";
import { useCallback, useMemo } from "react";
import { indexOf } from "../../util";

export type ValueLabelProps = React.DetailedHTMLProps<
  React.HTMLAttributes<HTMLDivElement>,
  HTMLDivElement
> & {
  checked: boolean;
  label: string;
  ref: React.Ref<HTMLDivElement>;
};

export type ValueLabel = React.FC<ValueLabelProps>;

export const ValueLabelInternal = ({
  label,
  index,
  selectedIndex,
  numValues,
  selectIndex,
  radios,
  displayFormatter,
  ClientComponent,
}: {
  label: string;
  index: number;
  selectedIndex: number | undefined;
  numValues: number;
  selectIndex?: (i: number) => void;
  radios: React.RefObject<Map<number, HTMLDivElement>>;
  displayFormatter?: (value: string) => string;
  ClientComponent: ValueLabel;
}) => {
  const displayLabel = useMemo(
    () => displayFormatter?.(label) ?? label,
    [displayFormatter, label],
  );
  const onKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      const doSelect = (index: number) => {
        e.preventDefault();
        e.stopPropagation();
        selectIndex?.(index);
      };
      if (e.key === "Space" || e.key === " ") {
        doSelect(index);
        return;
      }
      const directions: Record<string, number> = {
        Up: -1,
        ArrowUp: -1,
        Down: 1,
        ArrowDown: 1,
        Left: -1,
        ArrowLeft: -1,
        Right: 1,
        ArrowRight: 1,
      };
      const direction = directions[e.key];
      if (direction !== undefined) {
        const newIndex = (index + direction + numValues) % numValues;
        doSelect(newIndex);
      }

      if (e.key === "Home") {
        doSelect(0);
      }
      if (e.key === "End") {
        doSelect(numValues - 1);
      }
    },
    [index, numValues, selectIndex],
  );

  const onClick = useCallback(() => {
    selectIndex?.(index);
  }, [index, selectIndex]);

  const onRef = useCallback(
    (el: HTMLDivElement | null) => {
      if (el === null) {
        radios.current.delete(index);
      } else {
        radios.current.set(index, el);
      }
    },
    [index, radios],
  );

  const checked = selectedIndex === index;

  return (
    <ClientComponent
      className="relative"
      onClick={onClick}
      onKeyDown={onKeyDown}
      role={"radio"}
      aria-label={displayLabel}
      aria-checked={checked}
      tabIndex={(selectedIndex ?? 0) == index ? 0 : -1}
      ref={onRef}
      checked={checked}
      label={displayLabel}
    ></ClientComponent>
  );
};

export type LabelGroupProps = {
  accessibilityLabel: string;
  values: string[];
  value?: string;
  displayFormatter?: (value: string) => string;
  valueLabel: ValueLabel;
  radios: React.RefObject<Map<number, HTMLDivElement>>;
  selectIndex: (i: number) => void;
};

export const LabelGroup = ({
  accessibilityLabel,
  value,
  values,
  displayFormatter,
  valueLabel,
  radios,
  selectIndex,
}: LabelGroupProps) => {
  const index = indexOf(value, values);

  return (
    <div
      className="inline-block"
      role="radiogroup"
      aria-label={accessibilityLabel}
    >
      {values.map((v, i) => (
        <ValueLabelInternal
          key={`choice-${i}`}
          label={v}
          index={i}
          selectedIndex={index}
          numValues={values.length}
          selectIndex={selectIndex}
          radios={radios}
          displayFormatter={displayFormatter}
          ClientComponent={valueLabel}
        />
      ))}
    </div>
  );
};
