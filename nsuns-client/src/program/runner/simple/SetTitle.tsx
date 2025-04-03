import { Component, createMemo, For, Match, Show, Switch } from "solid-js";
import { Movement, ProgramSet } from "../../../api";
import { repsDisplay, resolvedWeightDisplay, resolveWeight } from "../../../util/setDisplay";
import { currentSet, day } from "../state";
import { getLatestMax } from "../../../hooks/useMovementsToMaxesMap";
import { useProgram } from "../context/ProgramProvider";
import { createSmartAsyncDelay } from "../../../hooks/asymmetricDelay";
import { calculatePlates } from "../../../util/plates";

export const SetTitle: Component<{
  current?: ProgramSet;
  currentMovement?: Movement;
  currentMax?: number;
}> = (props) => {
  const weightDisplay = () => {
    const currentSet = props.current;
    return currentSet ? resolvedWeightDisplay(currentSet, props.currentMax) : "";
  };
  const repDisplay = () => {
    const currentSet = props.current;
    return currentSet ? repsDisplay(currentSet) : "";
  };

  return (
    <>
      <h1 class="text-9xl mb-4">
        <Show
          when={props.current && props.currentMovement}
          fallback="Rest"
        >
          {props.currentMovement?.name}
          <br />
          <Show when={weightDisplay()}>
            {weightDisplay()}
            <br />
          </Show>
          <Show when={repDisplay()}>
            {repDisplay()}
            <br />
          </Show>
          <Show when={props.current?.description}>{props.current?.description}</Show>
        </Show>
      </h1>
    </>
  );
};

export const LoadingTitle: Component = () => {
  return (
    <>
      <h1 class="h-48 py-1">
        <div class="h-full w-full shimmer rounded" />
      </h1>
      <h2 class="h-24 py-1">
        <div class="h-full w-full shimmer rounded" />
      </h2>
    </>
  );
};

export const TitleBanner: Component = () => {
  const { getSets, movementMap, movementsToMaxesMap, queryState } = useProgram();

  const currentProgramSet = () => getSets(day())[currentSet()];
  const currentMovement = () => {
    const set = currentProgramSet();
    return set && movementMap()[set.movementId];
  };
  const currentMax = () => {
    const set = currentProgramSet();
    return set ? getLatestMax(movementsToMaxesMap(), set)?.amount : undefined;
  };
  const plates = createMemo(() => {
    const set = currentProgramSet();
    const max = currentMax();
    const weight = set && max && resolveWeight(set, max);

    if (weight) {
      return calculatePlates(weight, 45, [45, 25, 10, 5, 2.5]);
    }

    return [];
  });

  const isLoading = createSmartAsyncDelay(queryState.isLoading);

  return (
    <Switch>
      <Match when={isLoading()}>
        <LoadingTitle />
      </Match>
      <Match when={queryState.isSuccess()}>
        <SetTitle
          current={currentProgramSet()}
          currentMovement={currentMovement()}
          currentMax={currentMax()}
        />
        <Show when={plates().length}>
          <div class="flex flex-row gap-4 text-4xl">
            <span>Plates:</span>
            <For each={plates()}>
              {(plate) => (
                <span>
                  {plate.count} x {plate.weight}
                </span>
              )}
            </For>
          </div>
        </Show>
        <Show when={getSets(day()).length}>
          <div class="text-6xl text-gray-400">
            Set {currentSet() + 1} of {getSets(day()).length}
          </div>
        </Show>
      </Match>
    </Switch>
  );
};
