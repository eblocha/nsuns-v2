import { A } from "@solidjs/router";
import { Component } from "solid-js";
import { Home } from "../../icons/Home";
import { ChevronDown } from "../../icons/ChevronDown";

export const ProgramRunner: Component = () => {
  return (
    <div class="w-full h-full overflow-hidden flex flex-col">
      <nav class="w-full flex flex-row items-center p-2 flex-shrink-0 gap-2">
        <A href="/" class="text-button">
          <Home />
        </A>
        <A href="../" class="text-button">
          <ChevronDown class="rotate-90" />
        </A>
      </nav>
      <div class="flex-grow">

      </div>
    </div>
  );
};
