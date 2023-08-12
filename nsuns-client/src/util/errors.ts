export const displayError = (err: unknown, action: string) => {
  // eslint-disable-next-line @typescript-eslint/no-base-to-string
  const errString = err ? `: ${err}` : "";

  return `Failed to ${action}${errString}`;
};
