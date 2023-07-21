import { Component, For, Show, createMemo, createSignal } from "solid-js";
import { Movement, ProgramSet } from "../../api";
import { Plus } from "../../icons/Plus";
import { NewSet } from "./NewSet";
import { useMovementsQuery } from "../../hooks/queries/movements";

const dayNames = [
  "Sunday",
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
];

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

export const Days: Component<{ sets: ProgramSet[]; programId: number }> = (
  props
) => {
  const [addSetTo, setAddSetTo] = createSignal<number | null>(null);
  const query = useMovementsQuery();

  const setMap = createMemo(() => {
    const m: Record<string, ProgramSet[]> = {};
    dayNames.forEach((name, index) => {
      m[name] = props.sets.filter((set) => set.day === index);
    });
    return m;
  });

  const movements = () => query.data ?? [];

  return (
    <ul>
      <For each={dayNames}>
        {(day, index) => {
          return (
            <li class="mb-4">
              <h3 class="text-lg">
                {day}
                <Show when={!setMap()[day]?.length}>
                  <span class="italic opacity-80 text-sm ml-4">Rest Day</span>
                </Show>
              </h3>
              <ul>
                <For each={setMap()[day]}>
                  {(set) => (
                    <li class="rounded p-2 border border-gray-700 mb-2">
                      {displaySet(set, movements())}
                    </li>
                  )}
                </For>
                <Show when={addSetTo() === index()}>
                  <li>
                    <NewSet
                      close={() => setAddSetTo(null)}
                      dayIndex={index()}
                      programId={props.programId}
                      movements={movements()}
                    />
                  </li>
                </Show>
                <li>
                  <button
                    class="text-button text-sm border border-gray-700 flex flex-row items-center justify-center gap-2"
                    disabled={addSetTo() !== null}
                    onClick={() => setAddSetTo(index())}
                  >
                    <Plus />
                    Add Set
                  </button>
                </li>
              </ul>
            </li>
          );
        }}
      </For>
    </ul>
  );
};
