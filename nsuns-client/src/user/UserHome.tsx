import { A, useParams } from "@solidjs/router";
import { Component, For, Match, Switch } from "solid-js";
import { getUserPrograms } from "../api/program";
import { AddProgram, LoadingProgram, ProgramItem } from "./Program";
import { createQuery } from "@tanstack/solid-query";
import { UserGreeting } from "./UserGreeting";

export const UserHome: Component = () => {
  const params = useParams<{ id: string }>();
  const programsQuery = createQuery({
    queryKey: () => ["programs", params.id],
    queryFn: () => getUserPrograms(params.id),
    enabled: !!params.id,
  });

  return (
    <div class="h-full w-1/3 pt-60 px-32">
      <div class="mb-4">
        <UserGreeting id={params.id} />
      </div>
      <h2 class="text-lg">Your Programs</h2>
      <div class="flex flex-col justify-center">
        <Switch>
          <Match when={programsQuery.isLoading}>
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
          <Match when={programsQuery.isError}>
            <div class="flex flex-col items-center justify-center">
              <div class="mb-2">Error</div>
            </div>
          </Match>
          <Match when={programsQuery.isSuccess}>
            <ul class="my-4 w-full">
              <For each={programsQuery.data?.all}>
                {(program, i) => (
                  <li>
                    <ProgramItem
                      program={program}
                      index={i()}
                      isDefault={program.id === programsQuery.data?.default?.id}
                    />
                  </li>
                )}
              </For>
            </ul>
          </Match>
        </Switch>
        <div class="flex flex-row items-center">
          <AddProgram />
          <button
            onClick={() => programsQuery.refetch()}
            disabled={programsQuery.isFetching}
            class="secondary-button ml-2"
          >
            Refresh
          </button>
          <A
            href="/"
            class="secondary-button ml-2"
          >
            Home
          </A>
        </div>
      </div>
    </div>
  );
};
