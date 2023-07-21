import { Component, For, JSX, Show } from "solid-js";
import { Control } from "../hooks/forms";

export const ErrorMessages: Component<
  JSX.HTMLAttributes<HTMLElement> & { control: Control<any> }
> = (props) => {
  return (
    <Show when={props.control.showErrors()}>
      <For each={props.control.errorMessages()}>
        {(msg) => (
          <small {...props} class={`text-red-500 ${props.class}`}>
            {msg}
          </small>
        )}
      </For>
    </Show>
  );
};
