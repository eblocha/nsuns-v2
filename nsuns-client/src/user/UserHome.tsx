import { useParams } from "@solidjs/router";
import { Component, For, Match, Show, Switch, createResource } from "solid-js";
import { getUserPrograms } from "../api/program";
import { AddProgram, LoadingProgram, ProgramItem } from "./Program";
import { createQuery } from "@tanstack/solid-query";

const Error: Component<{ message: string }> = (props) => {
  return (
    <div class="flex flex-col items-center justify-center">
      <div class="mb-2">Error: {props.message}</div>
    </div>
  );
};

const LoadingOrError: Component<{
  error?: unknown;
  isLoading: boolean;
}> = (props) => {
  return (
    <Show
      when={props.isLoading}
      fallback={<Error message={`${props.error}`} />}
    >
      <ul class="my-8 w-full">
        <For each={[1, 2, 3]}>
          {() => (
            <li>
              <LoadingProgram />
            </li>
          )}
        </For>
      </ul>
    </Show>
  );
};

export const UserHome: Component = () => {
  const params = useParams<{ id: string }>();
  const query = createQuery({
    queryKey: () => ["programs", params.id],
    queryFn: () => getUserPrograms(params.id),
    enabled: !!params.id,
  });

  return (
    <div class="h-full w-full p-60">
      <h2 class="text-lg">Your Programs</h2>
      <div class="flex flex-col items-center justify-center">
        <Switch>
          <Match when={query.isLoading}>
            <ul class="my-8 w-full">
              <For each={[1, 2, 3]}>
                {() => (
                  <li>
                    <LoadingProgram />
                  </li>
                )}
              </For>
            </ul>
          </Match>
          <Match when={query.isError}>
            <div class="flex flex-col items-center justify-center">
              <div class="mb-2">Error</div>
            </div>
          </Match>
          <Match when={query.isSuccess}>
            <ul class="my-8 w-full">
              <For each={query.data?.all}>
                {(program, i) => (
                  <li>
                    <ProgramItem
                      program={program}
                      index={i()}
                      isDefault={program.id === query.data?.default?.id}
                    />
                  </li>
                )}
              </For>
              <li>
                <AddProgram />
              </li>
            </ul>
          </Match>
        </Switch>
        <div class="flex flex-row items-center justify-center">
          <button
            onClick={() => query.refetch()}
            disabled={query.isFetching}
            class="p-3 rounded bg-gray-300 hover:bg-gray-400 active:bg-gray-500 mr-2"
          >
            Refresh
          </button>
        </div>
      </div>
    </div>
  );
};
