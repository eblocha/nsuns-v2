import { Show, type Component, createRenderEffect } from "solid-js";
import { IFormControl } from "solid-forms";
import style from "./TextInput.module.css";

export const TextInput: Component<{
  control: IFormControl<string>;
  name?: string;
  type?: string;
  class?: string;
}> = (props) => {

  createRenderEffect(() => {
    if (!props.control.isRequired || props.control.value.length > 0) {
      props.control.setErrors(null);
    } else {
      props.control.setErrors({ isMissing: true });
    }
  });

  const showErrors = () => {
    return !!props.control.errors && props.control.isDirty;
  }

  return (
    <div class="flex flex-col items-end">
      <input
        class={props.class}
        classList={{
          [style.invalid]: showErrors(),
        }}
        name={props.name}
        type={props.type}
        value={props.control.value}
        oninput={(e) => {
          props.control.setValue(e.currentTarget.value);
          props.control.markDirty(true);
        }}
        onblur={() => props.control.markTouched(true)}
        disabled={props.control.isDisabled}
        required={props.control.isRequired}
      />
      <Show when={showErrors() && props.control.errors?.isMissing}>
        <small class="text-red-500">This field is required.</small>
      </Show>
    </div>
  );
};
