import { Component, For, Match, Switch, createMemo } from "solid-js";
import { Graph } from "../../../graph/Graph";
import { plural } from "../../../util/setDisplay";
import { useStats } from "../../../stats/StatsProvider";
import { Reps } from "../../../api/reps";
import { Max } from "../../../api/maxes";
import { Movement } from "../../../api";

const displayAmount = (amount?: number) => {
  return amount !== undefined ? `${amount} lb${plural(amount)}` : "None";
};

const displayReps = (amount?: number) => {
  return amount !== undefined ? `${amount} rep${plural(amount)}` : "no reps";
};

type MovementData = {
  movement: Movement;
  maxes: Max[];
  reps: Reps[] | undefined;
};

export const StatList: Component = () => {
  const {
    queryState: { isLoading, isSuccess, isError, error },
    movementsToMaxesMap,
    movementsToRepsMap,
    movementMap,
  } = useStats();

  const maxesData = createMemo(() => {
    return Object.values(movementMap())
      .map((movement) => {
        const maxes = movementsToMaxesMap()[movement.id];
        const reps = movementsToRepsMap()[movement.id];

        return { movement, maxes, reps };
      })
      .filter((data): data is MovementData => !!data.maxes);
  });

  return (
    <ul>
      <Switch>
        <Match when={isLoading()}>
          <li>
            <div class="shimmer w-full h-20" />
          </li>
        </Match>
        <Match when={isError()}>Failed to fetch stats: {`${error()}`}</Match>
        <Match when={isSuccess()}>
          <For each={maxesData()}>
            {(entry) => (
              <li class="w-full grid grid-cols-4 h-20 gap-2 mt-2">
                <div class="flex flex-col items-center justify-center">
                  {entry.movement.name}
                </div>
                <div class="col-span-2 h-20 text-blue-500 border-l border-b border-gray-600 p-1">
                  <Graph
                    data={entry.maxes.map((max, index) => ({
                      x: index,
                      y: max.amount,
                    }))}
                    weight={4}
                    fillOpacity="10%"
                  />
                </div>
                <div class="flex flex-col items-start justify-center">
                  <p>
                    {displayAmount(
                      entry.maxes[entry.maxes.length - 1]?.amount
                    ) +
                      " " +
                      displayReps(entry.reps?.[entry.reps.length - 1]?.amount ?? undefined)}
                  </p>
                </div>
              </li>
            )}
          </For>
        </Match>
      </Switch>
    </ul>
  );
};
