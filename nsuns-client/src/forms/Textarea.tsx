import { Show, type Component, createRenderEffect, JSXElement } from "solid-js";
import { IFormControl } from "solid-forms";
import style from "./TextInput.module.css";
import { hasErrors } from "./errors";

export const TextArea: Component<{
  control: IFormControl<string>;
  name?: string;
  class?: string;
  onBlur?: (
    e: FocusEvent & {
      currentTarget: HTMLTextAreaElement;
      target: HTMLTextAreaElement;
    }
  ) => void;
  children?: JSXElement;
}> = (props) => {
  createRenderEffect(() => {
    if (!props.control.isRequired || props.control.value.length > 0) {
      props.control.patchErrors({ isMissing: false });
    } else {
      props.control.patchErrors({ isMissing: true });
    }
  });

  const showErrors = () => {
    return hasErrors(props.control.errors) && props.control.isDirty;
  };

  return (
    <div class="flex flex-col items-end">
      <textarea
        class={props.class}
        classList={{
          [style.invalid]: showErrors(),
        }}
        name={props.name}
        value={props.control.value}
        oninput={(e) => {
          props.control.setValue(e.currentTarget.value);
          props.control.markDirty(true);
        }}
        onblur={(e) => {
          props.control.markTouched(true);
          props.onBlur?.(e);
        }}
        disabled={props.control.isDisabled}
        required={props.control.isRequired}
      />
      <Show when={showErrors() && props.control.errors?.isMissing}>
        <small class="text-red-500">This field is required.</small>
      </Show>
      {props.children}
    </div>
  );
};
