import { A, useParams } from "@solidjs/router";
import { Component, createEffect, on } from "solid-js";
import { Home } from "../../icons/Home";
import { ChevronDown } from "../../icons/ChevronDown";
import { useProgramSummaryQuery } from "../../hooks/queries/sets";
import { useSetMap } from "../../hooks/useSetMap";
import { useMovementsQuery } from "../../hooks/queries/movements";
import { ArrowRight } from "../../icons/ArrowRight";
import { decrementDay, incrementDay, setCurrentSet } from "./state";
import { AnimatedSetList } from "./AnimatedSetList";

export const ProgramRunner: Component = () => {
  const params = useParams<{ programId: string; profileId: string }>();

  const query = useProgramSummaryQuery(params.programId);
  const movementsQuery = useMovementsQuery();

  const setMap = useSetMap(() => query.data?.sets ?? []);

  createEffect(
    on(
      () => params.profileId,
      () => {
        setCurrentSet(0);
      }
    )
  );

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
      <div class="flex-grow p-5 overflow-hidden grid grid-cols-3">
        <div class="h-full overflow-hidden flex flex-row items-center">
          <button
            class="w-10 h-10 m-2 circle-text-button flex flex-row items-center justify-center"
            onClick={decrementDay}
          >
            <ArrowRight class="rotate-180" />
          </button>
          <div class="flex-grow overflow-hidden h-full relative">
            <AnimatedSetList
              setMap={setMap()}
              movements={movementsQuery.data}
            />
          </div>
          <button
            class="w-10 h-10 m-2 circle-text-button flex flex-row items-center justify-center"
            onClick={incrementDay}
          >
            <ArrowRight />
          </button>
        </div>
      </div>
    </div>
  );
};
