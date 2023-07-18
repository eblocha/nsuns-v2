import { A, Outlet, useNavigate, useParams } from "@solidjs/router";
import { Component, For, Match, Switch, createEffect } from "solid-js";
import { getUserPrograms } from "../api/program";
import { AddProgram, LoadingProgram, ProgramItem } from "./Program";
import { createQuery } from "@tanstack/solid-query";
import { UserGreeting } from "./UserGreeting";

export const UserHome: Component = () => {
  const params = useParams<{ userId: string }>();
  const navigate = useNavigate();
  const programsQuery = createQuery({
    queryKey: () => ["programs", params.userId],
    queryFn: () => getUserPrograms(params.userId),
    enabled: !!params.userId,
  });

  createEffect(() => {
    if (programsQuery.isSuccess && programsQuery.data?.all.length === 0) {
      navigate("program/new");
    }
  });

  return (
    <div class="h-full grid grid-cols-3 gap-10">
      <div class="h-full py-60 pl-32 pr-12">
        <div class="mb-4">
          <UserGreeting id={params.userId} />
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
              <div class="flex flex-col items-center justify-center my-10">
                <div class="mb-2">
                  Error fetching programs: {`${programsQuery.error}`}
                </div>
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
                        isDefault={
                          program.id === programsQuery.data?.default?.id
                        }
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
            <A href="/" class="text-button ml-2">
              Home
            </A>
          </div>
        </div>
      </div>
      <div class="h-full col-span-2">
        <Outlet />
      </div>
    </div>
  );
};
