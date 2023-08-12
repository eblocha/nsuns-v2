import { Component, JSX } from "solid-js";
import { Spinner } from "../icons/Spinner";

export const RefreshButton: Component<
  JSX.ButtonHTMLAttributes<HTMLButtonElement> & {
    isFetching: boolean;
  }
> = (props) => {
  return (
    <button
      {...props}
      disabled={props.isFetching || props.disabled}
      class={`flex flex-row items-center justify-center w-20 ${props.class}`}
    >
      {props.isFetching ? <Spinner class="animate-spin my-1" /> : "Refresh"}
    </button>
  );
};
