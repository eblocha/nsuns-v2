import { Component, JSX, Show } from "solid-js";
import { Portal } from "solid-js/web";

export const Modal: Component<{
  children?: JSX.Element;
  open?: boolean;
  onBackdropClick?: JSX.EventHandler<HTMLDivElement, MouseEvent>;
}> = (props) => {
  return (
    <Show when={props.open}>
      <Portal>
        <div class="w-screen h-screen bg-black opacity-50 absolute top-0 left-0"></div>
        <div
          class="w-screen h-screen overflow-hidden flex flex-row items-center justify-center absolute top-0 left-0"
          onClick={props.onBackdropClick}
        >
          {props.children}
        </div>
      </Portal>
    </Show>
  );
};
