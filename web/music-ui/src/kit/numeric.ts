/**
 * Props shared for all numeric controls
 */
export type Props = {
  /**
   * The current value of the control (scaled to 0-100)
   */
  value: number;

  /**
   * True if the control is grabbed
   */
  grabbed?: boolean;

  /**
   * Callback for when the value of the control changes.
   * Note that this may be called spuriously even if the value didn't change.
   */
  onValue?: (value: number) => void;

  /**
   * Callback for when the control is grabbed or release through a pointer event.
   * Note that this may be called spruriously even if the grabbed state didn't change.
   */
  onGrabOrRelease?: (grabbed: boolean) => void;

  /**
   * Label for accessibility
   */
  accessibilityLabel: string;

  /**
   * Value to reset the control to on reset-to-default gesture
   */
  defaultValue?: number;

  /**
   * Value formatter to convert values into strings
   */
  valueFormatter?: (value: number) => string;
};

/**
 * Props for numeric controls with an optional visible label
 */
export type PropsWithLabel = Omit<Props, "accessibilityLabel"> & {
  /**
   * Whether we should show the label
   */
  showLabel?: boolean;

  /**
   * The label of the control
   */
  label: string;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;
};
