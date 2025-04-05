import { Component, Match, Show, Switch } from "solid-js";
import { Movement, ProgramSet } from "../../../api";
import { repsDisplay, resolvedWeightDisplay, resolveWeight } from "../../../util/setDisplay";
import { currentSet, day } from "../state";
import { getLatestMax } from "../../../hooks/useMovementsToMaxesMap";
import { useProgram } from "../context/ProgramProvider";
import { createSmartAsyncDelay } from "../../../hooks/asymmetricDelay";
import { PlateGraphic } from "../PlateGraphic";

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
    <h1 class="text-9xl overflow-auto flex-grow">
      <Show
        when={props.current && props.currentMovement}
        fallback="Rest"
      >
        {props.currentMovement?.name}
        <Show when={weightDisplay()}>
          <br />
          {weightDisplay()}
        </Show>
        <Show when={repDisplay()}>
          <br />
          {repDisplay()}
        </Show>
        <Show when={props.current?.description}>
          <br />
          {props.current?.description}
        </Show>
      </Show>
    </h1>
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
  const currentWeight = () => {
    const set = currentProgramSet();
    const max = currentMax();
    return set && max && resolveWeight(set, max);
  };

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
        <PlateGraphic weight={currentWeight()} />
      </Match>
    </Switch>
  );
};
