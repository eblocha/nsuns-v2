import { A, useParams } from "@solidjs/router";
import { Component, Match, Switch, createEffect, on } from "solid-js";
import { useProgramSummaryQuery } from "../../hooks/queries/sets";
import { useSetMap } from "../../hooks/useSetMap";
import { useMovementsQuery } from "../../hooks/queries/movements";
import { ArrowRight } from "../../icons/ArrowRight";
import { dayName, decrementDay, incrementDay, setCurrentSet } from "./state";
import { AnimatedSetList } from "./AnimatedSetList";
import { LoadingTitle, TitleBanner } from "./SetTitle";
import { useMovementMap } from "../../hooks/useMovementMap";
import { useMaxesQuery } from "../../hooks/queries/maxes";
import { useMovementsToMaxesMap } from "../../hooks/useMovementsToMaxesMap";
import { Edit } from "../../icons/Edit";
import { User } from "../../icons/User";
import { Tools } from "./Tools";

export const ProgramRunner: Component = () => {
  const params = useParams<{ programId: string; profileId: string }>();

  const summaryQuery = useProgramSummaryQuery(() => params.programId);
  const movementsQuery = useMovementsQuery();
  const maxesQuery = useMaxesQuery(() => params.profileId);

  const isLoading = () =>
    summaryQuery.isLoading || movementsQuery.isLoading || maxesQuery.isLoading;
  const isSuccess = () =>
    summaryQuery.isSuccess && movementsQuery.isSuccess && maxesQuery.isSuccess;

  const setMap = useSetMap(() => summaryQuery.data?.sets ?? []);
  const movementMap = useMovementMap(() => movementsQuery.data ?? []);
  const movementsToMaxesMap = useMovementsToMaxesMap(
    () => maxesQuery.data ?? []
  );

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
            <User />
          </A>
          <A href="../" class="text-button">
            <Edit />
          </A>
        </nav>
        <div class="px-6 flex-grow">
          <Switch>
            <Match when={isLoading()}>
              <LoadingTitle />
            </Match>
            <Match when={isSuccess()}>
              <TitleBanner
                setMap={setMap()}
                movementMap={movementMap()}
                movementsToMaxesMap={movementsToMaxesMap()}
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
        <div class="col-span-2 h-full flex flex-col gap-4">
          <div class="flex-grow w-full"></div>
          <Tools nSets={setMap()[dayName()].length ?? 0} />
        </div>
      </div>
    </div>
  );
};
