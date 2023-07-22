import { Component, For, Match, Show, Switch, createSignal } from "solid-js";
import { CreateMovement } from "./CreateMovement";
import { Plus } from "../icons/Plus";
import { Movement } from "./Movement";
import { useMovementsQuery } from "../hooks/queries/movements";

const Loading: Component = () => {
  return (
    <div class="w-full h-full grid grid-rows-6 gap-4">
      <div class="shimmer rounded" />
      <div class="shimmer rounded" />
      <div class="shimmer rounded" />
      <div class="shimmer rounded" />
      <div class="shimmer rounded" />
      <div class="shimmer rounded" />
    </div>
  );
};

export const MovementList: Component = () => {
  const query = useMovementsQuery();
  const [showForm, setShowForm] = createSignal(false);

  return (
    <div class="w-full h-full flex flex-col">
      <h2 class="mb-4 text-xl">Movements</h2>
      <div class="flex-grow">
        <Switch>
          <Match when={query.isLoading}>
            <Loading />
          </Match>
          <Match when={query.isError}>
            <div class="h-full w-full flex flex-col items-center">
              Failed to fetch movements: {`${query.error}`}
            </div>
          </Match>
          <Match when={query.isSuccess}>
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
                <For each={query.data}>
                  {(movement) => <Movement {...movement} />}
                </For>
              </ul>
            </div>
          </Match>
        </Switch>
      </div>
    </div>
  );
};
