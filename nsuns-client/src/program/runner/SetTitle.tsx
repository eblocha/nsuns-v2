import { Component } from "solid-js";
import { Movement, ProgramSet } from "../../api";

const displaySet = (set: ProgramSet, movement: Movement) => {
  const amountComponent = set.amount ? ` ${set.amount} lbs` : "";

  const reps = set.reps
    ? ` for ${set.reps} rep${set.reps === 1 ? "" : "s"}`
    : "";

  const description = !amountComponent && !reps ? `: ${set.description}` : "";

  return `${movement.name}${amountComponent}${reps}${description}`;
};

export const SetTitle: Component<{
  current?: ProgramSet;
  currentMovement?: Movement;
  next?: ProgramSet;
  nextMovement?: Movement;
}> = (props) => {
  return (
    <>
      <h1 class="text-8xl mb-4">
        {(props.current &&
          props.currentMovement &&
          displaySet(props.current, props.currentMovement)) ||
          "None"}
      </h1>
      <h2 class="text-5xl text-gray-400">
        Next:{" "}
        {(props.next &&
          props.nextMovement &&
          "Next: " + displaySet(props.next, props.nextMovement)) ||
          " None"}
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
