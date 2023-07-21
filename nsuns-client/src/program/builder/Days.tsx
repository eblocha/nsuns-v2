import { Component, For } from "solid-js";
import { ProgramSet } from "../../api";

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
            </ul>
          </li>
        )}
      </For>
    </ul>
  );
};
