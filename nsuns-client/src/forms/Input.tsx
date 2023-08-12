import { type Component, JSX } from "solid-js";
import style from "./Input.module.css";
import { Control } from "../hooks/forms";

export const Input: Component<
  JSX.InputHTMLAttributes<HTMLInputElement> & {
    control: Control<string>;
    onBlur?: JSX.FocusEventHandler<HTMLInputElement, FocusEvent>;
    onInput?: JSX.InputEventHandler<HTMLInputElement, InputEvent>;
  }
> = (props) => {
  return (
    <input
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
