import { Show, type Component } from "solid-js";
import { IFormControl } from "solid-forms";
import style from "./TextInput.module.css";

export const TextInput: Component<{
  control: IFormControl<string>;
  name?: string;
  type?: string;
  class?: string;
}> = (props) => {
  return (
    <>
      <input
        class={props.class}
        classList={{
          [style.invalid]: !!props.control.errors,
        }}
        name={props.name}
        type={props.type}
        value={props.control.value}
        oninput={(e) => {
          props.control.setValue(e.currentTarget.value);
        }}
        onblur={() => props.control.markTouched(true)}
        required={props.control.isRequired}
      />

      <Show when={props.control.isTouched && props.control.errors?.isMissing}>
        <small>Answer required.</small>
      </Show>
    </>
  );
};
