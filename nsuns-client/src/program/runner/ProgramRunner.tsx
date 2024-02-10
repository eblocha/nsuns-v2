import { A, useParams } from "@solidjs/router";
import { Component } from "solid-js";
import { ArrowRight } from "../../icons/ArrowRight";
import { decrementDay, incrementDay } from "./state";
import { AnimatedSetList } from "./AnimatedSetList";
import { TitleBanner } from "./SetTitle";
import { Edit } from "../../icons/Edit";
import { User } from "../../icons/User";
import { Tools } from "./Tools";
import { ProgramProvider } from "./context/ProgramProvider";
import { DataList } from "./data/DataList";

export const ProgramRunner: Component = () => {
  const params = useParams<{ programId: string; profileId: string }>();

  return (
    <ProgramProvider
      profileId={params.profileId}
      programId={params.programId}
    >
      <div class="w-full h-full overflow-hidden flex flex-col">
        <div class="w-full flex-shrink-0 flex flex-row">
          <nav class="flex flex-col items-center p-2 flex-shrink-0 gap-2">
            <A
              href="/"
              class="text-button"
            >
              <User />
            </A>
            <A
              href="../"
              class="text-button"
            >
              <Edit />
            </A>
          </nav>
          <div class="px-6 flex-grow">
            <TitleBanner />
          </div>
        </div>
        <div class="flex-grow p-5 overflow-hidden grid grid-rows-2 lg:grid-rows-1 lg:grid-cols-7 2xl:grid-cols-3">
          <div class="w-full lg:h-full overflow-hidden flex flex-row items-center lg:col-span-3 2xl:col-span-1">
            <button
              class="w-10 h-10 m-2 circle-text-button flex flex-row items-center justify-center"
              onClick={decrementDay}
            >
              <ArrowRight class="rotate-180" />
            </button>
            <div class="flex-grow overflow-hidden h-full relative">
              <AnimatedSetList />
            </div>
            <button
              class="w-10 h-10 m-2 circle-text-button flex flex-row items-center justify-center"
              onClick={incrementDay}
            >
              <ArrowRight />
            </button>
          </div>
          <div class="w-full lg:h-full lg:col-span-4 2xl:col-span-2 flex flex-col gap-4 overflow-hidden">
            <DataList />
            <Tools />
          </div>
        </div>
      </div>
    </ProgramProvider>
  );
};
