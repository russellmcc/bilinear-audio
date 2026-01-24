/**
 * Props for numeric controls with an optional visible label
 */
export type PropsWithLabel<P> = Omit<P, "accessibilityLabel"> & {
  /**
   * Whether and where we should show the label
   *  - "before": label comes before the control
   *  - "after": label comes after the control
   *  - "hidden": label is not shown
   *
   * Default is "after"
   */
  showLabel?: "before" | "after" | "hidden";

  /**
   * The label of the control
   */
  label: string;

  /**
   * Label for accessibility (can contain more information than `label`)
   */
  accessibilityLabel?: string;
};
