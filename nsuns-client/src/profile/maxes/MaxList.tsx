import { Component, For, Match, Switch, createMemo } from "solid-js";
import { useMaxesQuery } from "../../hooks/queries/maxes";
import { useMovementsToMaxesMap } from "../../hooks/useMovementsToMaxesMap";
import { useMovementsQuery } from "../../hooks/queries/movements";
import { Movement } from "../../api";
import { Max } from "../../api/maxes";
import { Graph } from "../../graph/Graph";
import { plural } from "../../util/setDisplay";

const displayAmount = (amount?: number) => {
  return amount !== undefined ? `${amount} lb${plural(amount)}` : "None"
}

export const MaxList: Component<{ profileId: string }> = (props) => {
  const maxesQuery = useMaxesQuery(() => props.profileId);
  const movementsQuery = useMovementsQuery();

  const isLoading = () => movementsQuery.isLoading || maxesQuery.isLoading;
  const isSuccess = () => movementsQuery.isSuccess && maxesQuery.isSuccess;

  const movementToMaxesMap = useMovementsToMaxesMap(
    () => maxesQuery.data ?? []
  );

  const maxesData = createMemo(() => {
    return Object.entries(movementToMaxesMap())
      .map(([movementId, maxes]) => {
        const movement = movementsQuery.data?.find(
          (m) => m.id.toString() === movementId
        );
        return { movement, maxes };
      })
      .filter((d): d is { movement: Movement; maxes: Max[] } => !!d.movement);
  });

  return (
    <ul>
      <Switch>
        <Match when={isLoading()}>Loading...</Match>
        <Match when={isSuccess()}>
          <For each={maxesData()}>
            {(entry) => (
              <li class="w-full grid grid-cols-4 h-20 gap-4 mt-2">
                <div class="flex flex-col items-center justify-center">{entry.movement.name}</div>
                <div class="col-span-2 h-20 text-blue-500 border-l border-b border-gray-600 p-1">
                  <Graph
                    data={entry.maxes.map((max, index) => ({
                      x: index,
                      y: max.amount,
                    }))}
                    weight={5}
                    style="line"
                  />
                </div>
                <div class="flex flex-col items-center justify-center">
                  <p>Current:</p>
                  <p>{displayAmount(entry.maxes[entry.maxes.length - 1]?.amount)}</p>
                </div>
              </li>
            )}
          </For>
        </Match>
      </Switch>
    </ul>
  );
};
