import { type Component, JSX } from "solid-js";
import style from "./Input.module.css";
import { Control } from "../hooks/forms";

export const TextArea: Component<
  JSX.TextareaHTMLAttributes<HTMLTextAreaElement> & {
    control: Control<string>;
    onBlur?: JSX.FocusEventHandler<HTMLTextAreaElement, FocusEvent>;
    onInput?: JSX.InputEventHandler<HTMLTextAreaElement, InputEvent>;
  }
> = (props) => {
  return (
    <textarea
      {...props}
      classList={{
        [style.invalid!]: props.control.showErrors(),
        ...props.classList,
      }}
      value={props.control.value()}
      onInput={(e) => {
        props.control.setValue(e.currentTarget.value);
        props.control.setDirty(true);
        props.onInput?.(e);
      }}
      onBlur={(e) => {
        props.control.setTouched(true);
        props.onBlur?.(e);
      }}
    />
  );
};
