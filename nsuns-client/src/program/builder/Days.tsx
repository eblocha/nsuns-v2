import { Component, For } from "solid-js";
import { ProgramSet } from "../../api";
import { Plus } from "../../icons/Plus";

const NoSets = () => {
  return <span class="italic opacity-80 text-sm">Rest Day</span>;
};

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
        {(day, index) => (
          <li class="mb-4">
            <h3 class="text-lg">{day}</h3>
            <ul>
              <For
                each={setsForDay(index())}
                fallback={
                  <li>
                    <NoSets />
                  </li>
                }
              >
                {(set) => <li>{set.description}</li>}
              </For>
              <li>
                <button class="text-button text-lg border border-gray-700 mt-2 flex flex-row items-center justify-center gap-2">
                  <Plus />
                  Add Set
                </button>
              </li>
            </ul>
          </li>
        )}
      </For>
    </ul>
  );
};
