import { A } from "@solidjs/router";
import { Component } from "solid-js";

export const NotFound: Component = () => {
  return (
    <div class="w-full h-full overflow-hidden flex flex-col items-center justify-center gap-8 p-8">
      <h1 class="text-3xl">404 Not Found</h1>
      <A
        href="/"
        class="primary-button"
      >
        Home
      </A>
    </div>
  );
};
