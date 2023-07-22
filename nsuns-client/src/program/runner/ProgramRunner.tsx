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
import { useMaxesQuery } from "../../hooks/queries/maxes";
import { useMovementsToMaxesMap } from "../../hooks/useMovementsToMaxesMap";

export const ProgramRunner: Component = () => {
  const params = useParams<{ programId: string; profileId: string }>();

  const summaryQuery = useProgramSummaryQuery(params.programId);
  const movementsQuery = useMovementsQuery();
  const maxesQuery = useMaxesQuery(params.profileId);

  const isLoading = () =>
    summaryQuery.isLoading || movementsQuery.isLoading || maxesQuery.isLoading;
  const isSuccess = () =>
    summaryQuery.isSuccess && movementsQuery.isSuccess && maxesQuery.isSuccess;

  const setMap = useSetMap(() => summaryQuery.data?.sets ?? []);
  const movementMap = useMovementMap(() => movementsQuery.data ?? []);
  const movementsToMaxesMap = useMovementsToMaxesMap(
    () => maxesQuery.data ?? []
  );

  const currentProgramSet = () => setMap()[dayName()]?.[currentSet()];
  const currentMovement = () => {
    const set = currentProgramSet();
    return set && movementMap()[set.movementId];
  };
  const currentMax = () => {
    const set = currentProgramSet();
    return set?.percentageOfMax
      ? movementsToMaxesMap()[set.percentageOfMax]?.amount
      : undefined;
  };

  const nextProgramSet = () => setMap()[dayName()]?.[currentSet() + 1];
  const nextMovement = () => {
    const set = nextProgramSet();
    return set && movementMap()[set.movementId];
  };
  const nextMax = () => {
    const set = nextProgramSet();
    return set?.percentageOfMax
      ? movementsToMaxesMap()[set.percentageOfMax]?.amount
      : undefined;
  };

  createEffect(
    on(
      () => params.profileId,
      () => {
        setCurrentSet(0);
      }
    )
  );

  createEffect(() => {
    console.log(movementsToMaxesMap());
  });

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
            <Match when={isLoading()}>
              <LoadingTitle />
            </Match>
            <Match when={isSuccess()}>
              <SetTitle
                current={currentProgramSet()}
                currentMovement={currentMovement()}
                currentMax={currentMax()}
                next={nextProgramSet()}
                nextMovement={nextMovement()}
                nextMax={nextMax()}
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
              movementMap={movementMap()}
              movementsToMaxesMap={movementsToMaxesMap()}
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
