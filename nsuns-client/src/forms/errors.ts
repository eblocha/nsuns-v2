import { ValidationErrors } from "solid-forms";

export const hasErrors = (errors: ValidationErrors | null) => {
  return !Object.values(errors ?? {}).every((v) => !v)
}