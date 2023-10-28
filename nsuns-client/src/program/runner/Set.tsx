import { Component, JSX, createEffect } from "solid-js";

export const SetComponent: Component<{
  onClick: () => void;
  isActive: boolean;
  children: JSX.Element;
}> = (props) => {
  let btn: HTMLButtonElement | undefined;

  createEffect(() => {
    if (props.isActive) {
      btn?.scrollIntoView({
        behavior: "smooth",
        block: "center",
      });
    }
  });

  return (
    <button
      ref={btn}
      onClick={props.onClick}
      class="text-button rounded w-full text-left"
      classList={{
        "text-button": !props.isActive,
        "primary-button": props.isActive,
      }}
    >
      {props.children}
    </button>
  );
};
