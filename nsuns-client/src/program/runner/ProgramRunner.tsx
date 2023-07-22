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
import { MaxesList } from "./maxes/MaxesList";

export const ProgramRunner: Component = () => {
  const params = useParams<{ programId: string; profileId: string }>();

  return (
    <ProgramProvider profileId={params.profileId} programId={params.programId}>
      <div class="w-full h-full overflow-hidden flex flex-col">
        <div class="w-full flex-shrink-0 flex flex-row">
          <nav class="flex flex-col items-center p-2 flex-shrink-0 gap-2">
            <A href="/" class="text-button">
              <User />
            </A>
            <A href="../" class="text-button">
              <Edit />
            </A>
          </nav>
          <div class="px-6 flex-grow">
            <TitleBanner />
          </div>
        </div>
        <div class="flex-grow p-5 overflow-hidden grid grid-cols-3">
          <div class="h-full overflow-hidden flex flex-row items-center">
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
          <div class="col-span-2 h-full flex flex-col gap-4 overflow-hidden">
            <div class="grid grid-cols-2">
              <div class="text-2xl">Program Maxes</div>
              {/* <div class="text-2xl">Reps</div> */}
            </div>
            <div class="flex-grow w-full overflow-auto">
              <MaxesList />
            </div>
            <Tools />
          </div>
        </div>
      </div>
    </ProgramProvider>
  );
};
