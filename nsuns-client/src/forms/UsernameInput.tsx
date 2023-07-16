import { IFormControl } from "solid-forms";
import {
  Accessor,
  Component,
  Setter,
  Show,
  createRenderEffect,
} from "solid-js";
import { TextInput } from "./TextInput";
import { isUsernameTaken } from "../api";
import { hasErrors } from "./errors";

export const UsernameInput: Component<{
  control: IFormControl<string>;
  validating: Accessor<boolean>;
  setValidating: Setter<boolean>;
  name?: string;
  type?: string;
  class?: string;
}> = (props) => {
  const setNoError = () => {
    props.control.patchErrors({ isTaken: false });
  };

  createRenderEffect(async () => {
    if (!props.control.value) {
      setNoError();
    }
  });

  const validate = async () => {
    if (!props.control.value) {
      setNoError();
      return;
    }
    props.setValidating(true);

    if (await isUsernameTaken(props.control.value)) {
      props.control.patchErrors({ isTaken: true });
    } else {
      setNoError();
    }

    props.setValidating(false);
  }

  const showErrors = () => {
    return hasErrors(props.control.errors) && props.control.isDirty;
  };

  return (
    <TextInput {...props} onBlur={validate}>
      <Show when={showErrors() && props.control.errors?.isTaken}>
        <small class="text-red-500">This username is taken.</small>
      </Show>
      <Show when={props.validating()}>
        <small>Validating...</small>
      </Show>
    </TextInput>
  );
};
