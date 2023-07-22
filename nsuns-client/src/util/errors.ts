export const displayError = (err: unknown, action: string) => {
  const errString = err ? `: ${err}` : "";

  return `Failed to ${action}${errString}`;
};
