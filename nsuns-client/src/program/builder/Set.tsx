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

  const weightComponent = percentOfMax?.name
    ? ` ${amountStr}% of ${percentOfMax.name} max`
    : ` ${amountStr} lb${plural(set.amount)}`;

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
        <button onClick={() => setIsEditing(true)} class="w-full h-full text-left text-button">
          {displaySet(props.set, props.movements)}
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
