import { A, useParams } from "@solidjs/router";
import { Component, Match, Switch, createEffect, on } from "solid-js";
import { Home } from "../../icons/Home";
import { ChevronDown } from "../../icons/ChevronDown";
import { useProgramSummaryQuery } from "../../hooks/queries/sets";
import { useSetMap } from "../../hooks/useSetMap";
import { useMovementsQuery } from "../../hooks/queries/movements";
import { ArrowRight } from "../../icons/ArrowRight";
import {
  currentSet,
  dayName,
  decrementDay,
  incrementDay,
  setCurrentSet,
} from "./state";
import { AnimatedSetList } from "./AnimatedSetList";
import { LoadingTitle, SetTitle } from "./SetTitle";
import { useMovementMap } from "../../hooks/useMovementMap";

export const ProgramRunner: Component = () => {
  const params = useParams<{ programId: string; profileId: string }>();

  const summaryQuery = useProgramSummaryQuery(params.programId);
  const movementsQuery = useMovementsQuery();

  const setMap = useSetMap(() => summaryQuery.data?.sets ?? []);

  const movementMap = useMovementMap(() => movementsQuery.data ?? []);

  const currentProgramSet = () => setMap()[dayName()]?.[currentSet()];
  const currentMovement = () => {
    const set = currentProgramSet();
    return set && movementMap()[set.movementId];
  };

  const nextProgramSet = () => setMap()[dayName()]?.[currentSet() + 1];
  const nextMovement = () => {
    const set = nextProgramSet();
    return set && movementMap()[set.movementId];
  };

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
      <div class="w-full flex-shrink-0 flex flex-row">
        <nav class="flex flex-col items-center p-2 flex-shrink-0 gap-2">
          <A href="/" class="text-button">
            <Home />
          </A>
          <A href="../" class="text-button">
            <ChevronDown class="rotate-90" />
          </A>
        </nav>
        <div class="px-6 flex-grow">
          <Switch>
            <Match when={summaryQuery.isLoading || movementsQuery.isLoading}>
              <LoadingTitle />
            </Match>
            <Match when={summaryQuery.isSuccess && movementsQuery.isSuccess}>
              <SetTitle
                current={currentProgramSet()}
                currentMovement={currentMovement()}
                next={nextProgramSet()}
                nextMovement={nextMovement()}
              />
            </Match>
          </Switch>
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
