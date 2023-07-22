import { Component, For, Setter, Show, createMemo } from "solid-js";
import { Movement, ProgramSet } from "../../api";
import { Dumbbell } from "../../icons/Dumbbell";
import { useMovementMap } from "../../hooks/useMovementMap";

type Section = {
  movement: Movement;
  sets: { set: ProgramSet; index: number }[];
};

const plural = (value: number) => (value === 1 ? "" : "s");

export const displaySet = (set: ProgramSet, max: number) => {
  const repsComponent =
    set.reps != null
      ? ` for ${set.reps}${set.repsIsMinimum ? "+" : ""} rep${
          set.repsIsMinimum ? "s" : plural(set.reps)
        }`
      : "";

  let weightComponent: string;

  if (set.percentageOfMax) {
    weightComponent = `${(set.amount / 100) * max} lbs`;
  } else {
    weightComponent = `${set.amount} lbs`;
  }

  return set.amount ? `${weightComponent}${repsComponent}` : set.description;
};

export const SetList: Component<{
  sets?: ProgramSet[];
  currentSet: number;
  setCurrentSet: Setter<number>;
  movements?: Movement[];
  day: string;
}> = (props) => {
  const movementMap = useMovementMap(() => props.movements ?? []);

  const sections = createMemo(() => {
    const sections: Section[] = [];
    const movements = movementMap();

    if (!props.sets) return sections;

    let currentSection: Section | null = null;
    let index = -1;
    for (const set of props.sets) {
      index++;
      const movement = movements[set.movementId];
      if (!movement) continue;

      if (!currentSection || currentSection.movement.id !== movement.id) {
        if (currentSection) sections.push(currentSection);

        currentSection = {
          movement: movement,
          sets: [{ set, index }],
        };
      } else if (currentSection.movement.id === movement.id) {
        currentSection.sets.push({ set, index });
      }
    }
    if (currentSection) sections.push(currentSection);

    return sections;
  });

  return (
    <div class="w-full h-full flex flex-col border rounded border-gray-700 overflow-hidden">
      <h2 class="text-xl border-b border-gray-700 p-2 bg-gray-800">
        {props.day}
      </h2>
      <div class="overflow-hidden flex-grow">
        <div class="h-full w-full overflow-auto">
          <ul class="p-2">
            <For
              each={sections()}
              fallback={
                <div class="w-full flex flex-col items-center justify-center text-lg">
                  <span class="italic">Rest day</span>
                  <Dumbbell class="mt-4 text-2xl" />
                </div>
              }
            >
              {(section, index) => (
                <>
                  <Show when={index() !== 0}>
                    <hr class="border-gray-600" />
                  </Show>
                  <h3
                    class="text-lg mb-1"
                    title={section.movement.description || undefined}
                  >
                    {section.movement.name}
                  </h3>
                  <For each={section.sets}>
                    {({ set, index }) => (
                      <li class="w-full mb-1 rounded">
                        <button
                          onClick={() => props.setCurrentSet(index)}
                          class="text-button rounded w-full text-left"
                          classList={{
                            "text-button": props.currentSet !== index,
                            "primary-button": props.currentSet === index,
                          }}
                        >
                          {displaySet(set, 100)}
                        </button>
                      </li>
                    )}
                  </For>
                </>
              )}
            </For>
          </ul>
        </div>
      </div>
    </div>
  );
};
