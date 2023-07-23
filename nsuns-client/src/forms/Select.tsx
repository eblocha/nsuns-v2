import { Component, For, JSX, createMemo, createRenderEffect } from "solid-js";
import { Control } from "../hooks/forms";
import style from "./Input.module.css";

export type SelectOption = {
  value: string;
  name?: string;
};

export const EMPTY_OPTION: SelectOption = {
  value: "",
  name: "",
};

export const Select: Component<
  JSX.SelectHTMLAttributes<HTMLSelectElement> & {
    control: Control<string>;
    options?: SelectOption[];
    emptyOption?: string;
    onBlur?: JSX.FocusEventHandler<HTMLSelectElement, FocusEvent>;
    onInput?: JSX.InputEventHandler<HTMLSelectElement, InputEvent>;
  }
> = (props) => {
  const options = createMemo(() => {
    if (!props.options?.length) {
      return [
        {
          value: "",
          name: props.emptyOption,
        },
      ];
    }

    if (props.emptyOption === undefined) {
      return props.options;
    }

    return [
      {
        value: "",
        name: props.emptyOption,
      },
      ...props.options,
    ];
  });

  createRenderEffect(() => {
    const firstOption = options()[0]?.value;
    if (!props.control.value() && firstOption) {
      props.control.setValue(firstOption);
    }
  });

  return (
    <select
      {...props}
      classList={{
        [style.invalid!]: props.control.showErrors(),
        ...props.classList,
      }}
      value={props.control.value()}
      onInput={(e) => {
        props.control.setValue(e.target.value);
        props.control.setDirty(true);
        props.onInput?.(e);
      }}
      onBlur={(e) => {
        props.control.setTouched(true);
        props.onBlur?.(e);
      }}
    >
      <For each={options()}>
        {(option) => (
          <option
            value={option.value}
            selected={option.value === props.control.value()}
          >
            {option.name ?? option.value}
          </option>
        )}
      </For>
    </select>
  );
};
