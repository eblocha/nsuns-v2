import { Component, For, Show } from "solid-js";
import { ProgramSet } from "../../api";
import { Plus } from "../../icons/Plus";

const dayNames = [
  "Sunday",
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
];

export const Days: Component<{ sets: ProgramSet[] }> = (props) => {
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
                <For each={sets}>{(set) => <li>{set.description}</li>}</For>
                <li>
                  <button class="text-button text-sm border border-gray-700 mt-2 flex flex-row items-center justify-center gap-2">
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
