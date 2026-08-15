export const displayError = (err: unknown, action: string) => {
  // oxlint-disable-next-line typescript/no-base-to-string typescript/restrict-template-expressions
  const errString = err ? `: ${err}` : "";

  return `Failed to ${action}${errString}`;
};
