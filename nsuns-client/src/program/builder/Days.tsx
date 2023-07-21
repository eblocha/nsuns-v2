import { Component, For, Show, createSignal } from "solid-js";
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

const displaySet = (set: ProgramSet, movements: Movement[]) => {
  const movement = movements.find((m) => m.id === set.movementId);

  const repsComponent =
    set.reps != null ? `for ${set.reps} Rep${set.reps === 1 ? "" : "s"}` : "";

  return `${movement ? movement.name : ""}${repsComponent}`;
};

export const Days: Component<{ sets: ProgramSet[]; programId: number }> = (
  props
) => {
  const [addSetTo, setAddSetTo] = createSignal<number | null>(null);
  const query = useMovementsQuery();

  const setsForDay = (day: number) => {
    return props.sets.filter((set) => set.day === day);
  };

  return (
    <ul>
      <For each={dayNames}>
        {(day, index) => {
          const sets = setsForDay(index());
          return (
            <li class="mb-4">
              <h3 class="text-lg">
                {day}
                <Show when={sets.length === 0}>
                  <span class="italic opacity-80 text-sm ml-4">Rest Day</span>
                </Show>
              </h3>
              <ul>
                <For each={sets}>
                  {(set) => <li>{displaySet(set, query.data ?? [])}</li>}
                </For>
                <Show when={addSetTo() === index()}>
                  <li>
                    <NewSet
                      close={() => setAddSetTo(null)}
                      dayIndex={index()}
                      programId={props.programId}
                      movements={query.data}
                    />
                  </li>
                </Show>
                <li>
                  <button
                    class="text-button text-sm border border-gray-700 mt-2 flex flex-row items-center justify-center gap-2"
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
