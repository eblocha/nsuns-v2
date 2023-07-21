import { Component, JSX } from "solid-js";
import { Control } from "../hooks/forms";

export const Checkbox: Component<
  JSX.InputHTMLAttributes<HTMLInputElement> & {
    control: Control<boolean>;
    onBlur?: JSX.FocusEventHandler<HTMLInputElement, FocusEvent>;
    onInput?: JSX.InputEventHandler<HTMLInputElement, InputEvent>;
  }
> = (props) => {
  return (
    <input
      {...props}
      type="checkbox"
      checked={props.control.value()}
      onInput={(e) => {
        props.control.setValue(e.currentTarget.checked);
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
