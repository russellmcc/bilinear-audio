export type Props = {
  /**
   * The possible values of the enum
   */
  values: string[];

  /**
   * True if the slider is grabbed
   */
  grabbed?: boolean;

  /**
   * The current value of the enum - must be one of `values`
   */
  value?: string;

  /**
   * The default value of the enum, if present, must be one of `values`
   */
  defaultValue?: string;

  /**
   * Accessibility label for the enum
   */
  accessibilityLabel: string;

  /**
   * Callback for when the value of the enum changes.
   */
  onValue?: (value: string) => void;

  /**
   * Callback for when the slider is grabbed or release through a pointer event.
   * Note that this may be called spruriously even if the grabbed state didn't change.
   */
  onGrabOrRelease?: (grabbed: boolean) => void;

  /**
   * Display formatter, if applicable. By default just shows the value.
   */
  displayFormatter?: (value: string) => string;
};
