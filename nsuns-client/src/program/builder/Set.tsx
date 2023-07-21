import { Component, Show, createSignal } from "solid-js";
import { Day, Movement, ProgramSet } from "../../api";
import { EditSet } from "./EditSet";

const plural = (value: number) => (value === 1 ? "" : "s");

const displaySet = (set: ProgramSet, movements: Movement[]) => {
  const movement = movements.find((m) => m.id === set.movementId);
  const percentOfMax = set.percentageOfMax
    ? movements.find((m) => m.id === set.percentageOfMax)
    : null;

  const amountStr = set.amount.toFixed(0);

  const nameComponent = movement ? movement.name : "";

  let weightComponent: string;

  if (percentOfMax) {
    const weightOfComponent =
      percentOfMax?.id === set.movementId
        ? " of max"
        : ` of ${percentOfMax?.name} max`;

    weightComponent = ` ${amountStr}%${weightOfComponent}`
  } else {
    weightComponent = set.amount ? ` ${amountStr} lb${plural(set.amount)}` : "";
  }

  const repsComponent =
    set.reps != null
      ? ` for ${set.reps}${set.repsIsMinimum ? "+" : ""} rep${
          set.repsIsMinimum ? "s" : plural(set.reps)
        }`
      : "";

  return `${nameComponent}${weightComponent}${repsComponent}`;
};

export const SetComponent: Component<{
  set: ProgramSet;
  movements: Movement[];
  dayIndex: Day;
  programId: number;
}> = (props) => {
  const [isEditing, setIsEditing] = createSignal(false);

  return (
    <Show
      when={isEditing()}
      fallback={
        <button
          onClick={() => setIsEditing(true)}
          class="w-full h-full text-left text-button"
        >
          {displaySet(props.set, props.movements)}
          <div class="text-sm opacity-60">{props.set.description}</div>
        </button>
      }
    >
      <EditSet
        close={() => setIsEditing(false)}
        set={props.set}
        dayIndex={props.dayIndex}
        programId={props.programId}
        movements={props.movements}
      />
    </Show>
  );
};
