import { Component } from "solid-js";
import { Movement, ProgramSet } from "../../api";
import { repsDisplay, resolvedWeightDisplay } from "../../util/setDisplay";

const displaySet = (set: ProgramSet, movement: Movement, max?: number) => {
  const weightComponent = resolvedWeightDisplay(set, max);
  const repsComponent = repsDisplay(set);

  const description =
    !weightComponent && !repsComponent && set.description ? `: ${set.description}` : "";

  return `${movement.name}${weightComponent}${repsComponent}${description}`;
};

export const SetTitle: Component<{
  current?: ProgramSet;
  currentMovement?: Movement;
  currentMax?: number;
  next?: ProgramSet;
  nextMovement?: Movement;
  nextMax?: number;
}> = (props) => {
  return (
    <>
      <h1 class="text-8xl mb-4">
        {(props.current &&
          props.currentMovement &&
          displaySet(props.current, props.currentMovement, props.currentMax)) ||
          "Rest"}
      </h1>
      <h2 class="text-5xl text-gray-400">
        Next:{" "}
        {(props.next &&
          props.nextMovement &&
          displaySet(props.next, props.nextMovement, props.nextMax)) ||
          " Rest"}
      </h2>
    </>
  );
};

export const LoadingTitle: Component = () => {
  return (
    <>
      <h1 class="h-28 py-1">
        <div class="h-full w-full shimmer rounded" />
      </h1>
      <h2 class="h-12 py-1">
        <div class="h-full w-full shimmer rounded" />
      </h2>
    </>
  );
};
