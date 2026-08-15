import { A, useParams } from "@solidjs/router";
import { Component, createSignal, Show } from "solid-js";

import { Edit } from "../../../icons/Edit";
import { Maximize } from "../../../icons/Maximize";
import { Minimize } from "../../../icons/Minimize";
import { User } from "../../../icons/User";
import { ProgramProvider } from "../context/ProgramProvider";
import { DataList } from "./data/DataList";
import { TitleBanner } from "./SetTitle";
import { Tools } from "./Tools";

const [isFullscreen, setIsFullscreen] = createSignal(!!document.fullscreenElement);

function toggleFullscreen() {
  if (!document.fullscreenElement) {
    setIsFullscreen(true);
    void document.documentElement.requestFullscreen();
  } else {
    setIsFullscreen(false);
    void document.exitFullscreen();
  }
}

export const SimpleProgramRunner: Component = () => {
  const params = useParams<{ programId: string; profileId: string }>();

  return (
    <ProgramProvider
      profileId={params.profileId}
      programId={params.programId}
    >
      <div class="w-full h-full shrink-0 flex flex-col gap-2">
        <div class="w-full grow flex flex-row overflow-auto">
          <nav class="flex flex-col items-center p-2 shrink-0 gap-2">
            <A
              href="/"
              class="text-button text-2xl"
            >
              <User />
            </A>
            <A
              href="../"
              class="text-button text-2xl"
            >
              <Edit />
            </A>
            <button
              class="text-button text-2xl"
              onClick={toggleFullscreen}
            >
              <Show
                when={isFullscreen()}
                fallback={<Maximize />}
              >
                <Minimize />
              </Show>
            </button>
          </nav>
          <div class="p-4 grow flex flex-col gap-4">
            <TitleBanner />
          </div>
          <div class="shrink-0 p-4 flex flex-col gap-4 w-60">
            <DataList />
          </div>
        </div>
        <div class="mt-auto p-4 shrink-0">
          <Tools />
        </div>
      </div>
    </ProgramProvider>
  );
};
