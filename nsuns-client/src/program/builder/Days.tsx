import { Component, For, Show, createMemo, createSignal } from "solid-js";
import { Day, Movement, ProgramSet } from "../../api";
import { Plus } from "../../icons/Plus";
import { NewSet } from "./NewSet";
import { useMovementsQuery } from "../../hooks/queries/movements";
import { SetComponent } from "./Set";

const dayNames = [
  "Sunday",
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
];

const EMPTY: never[] = [];

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

  const movements = () => query.data ?? EMPTY;

  return (
    <ul>
      <For each={dayNames}>
        {(day, index) => {
          return (
            <li class="mb-4">
              <h3 class="text-lg mb-2">
                {day}
                <Show when={!setMap()[day]?.length}>
                  <span class="italic opacity-80 text-sm ml-4">Rest Day</span>
                </Show>
              </h3>
              <ul>
                <For each={setMap()[day]}>
                  {(set) => (
                    <li class="rounded border border-gray-700 mb-2">
                      <SetComponent
                        set={set}
                        movements={movements()}
                        dayIndex={index() as Day}
                        programId={props.programId}
                      />
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
