import { Component, For, Match, Show, Switch, createSignal } from "solid-js";
import { CreateMovement } from "./CreateMovement";
import { Plus } from "../icons/Plus";
import { MovementItem } from "./Movement";
import { useStats } from "../stats/StatsProvider";
import { displayError } from "../util/errors";
import { createSmartAsyncDelay } from "../hooks/asymmetricDelay";

const Loading: Component = () => {
  return (
    <div class="w-full h-full grid grid-rows-6 gap-4">
      <div class="shimmer rounded h-12" />
      <div class="shimmer rounded h-12" />
      <div class="shimmer rounded h-12" />
      <div class="shimmer rounded h-12" />
      <div class="shimmer rounded h-12" />
      <div class="shimmer rounded h-12" />
    </div>
  );
};

export const MovementList: Component = () => {
  const { profileId, movementMap, movementsToMaxesMap, movementsToRepsMap, queryState } = useStats();
  const [showForm, setShowForm] = createSignal(false);

  const fetching = createSmartAsyncDelay(queryState.isFetching);

  return (
    <div class="w-full flex flex-col">
      <h2 class="mb-4 text-xl">Movements</h2>
      <div class="flex-grow">
        <Switch>
          <Match when={queryState.isLoading()}>
            <Loading />
          </Match>
          <Match when={queryState.isError()}>
            <div class="h-full w-full flex flex-col items-center">
              {displayError(queryState.error(), "fetch movements")}
            </div>
          </Match>
          <Match when={queryState.isSuccess()}>
            <div class="flex flex-col">
              <Show
                when={showForm()}
                fallback={
                  <button
                    class="text-button-outline flex flex-row items-center mb-2 gap-2"
                    onClick={() => setShowForm(true)}
                  >
                    <Plus />
                    <span>Add Movement</span>
                  </button>
                }
              >
                <div class="flex-shrink-0 mb-2">
                  <CreateMovement cancel={() => setShowForm(false)} />
                </div>
              </Show>
              <ul class="flex-grow">
                <For each={Object.values(movementMap())}>
                  {(movement) => (
                    <MovementItem
                      movement={movement}
                      profileId={profileId()}
                      maxes={movementsToMaxesMap()[movement.id]}
                      reps={movementsToRepsMap()[movement.id]}
                      isFetching={fetching()}
                    />
                  )}
                </For>
              </ul>
            </div>
          </Match>
        </Switch>
      </div>
    </div>
  );
};
