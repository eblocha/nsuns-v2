import { A, Outlet, useParams } from "@solidjs/router";
import { Component, For, Match, Switch, createEffect } from "solid-js";
import { AddProgram, LoadingProgram, ProgramItem } from "./Program";
import { ProfileGreeting } from "./ProfileGreeting";
import { createDelayedLatch } from "../hooks/createDelayedLatch";
import { RefreshButton } from "../components/RefreshButton";
import { useProgramsQuery } from "../hooks/queries/programs";
import { useNavigateToNewProgram } from "../hooks/navigation";

export const ProfileHome: Component = () => {
  const params = useParams<{ profileId: string; programId?: string }>();
  const navToNewProgram = useNavigateToNewProgram();
  const programsQuery = useProgramsQuery(params.profileId);

  createEffect(() => {
    if (programsQuery.isSuccess && programsQuery.data?.length === 0) {
      navToNewProgram();
    }
  });

  const isFetching = createDelayedLatch(() => programsQuery.isFetching, 400);

  return (
    <div class="h-full grid grid-cols-3 gap-10 overflow-hidden">
      <div class="h-full py-60 pl-32 pr-12">
        <div class="mb-4">
          <ProfileGreeting id={params.profileId} />
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
                <For each={programsQuery.data}>
                  {(program, i) => (
                    <li
                      class="rounded border mb-2"
                      classList={{
                        shimmer: isFetching(),
                        "border-blue-500":
                          program.id.toString() === params.programId,
                        "border-gray-600":
                          program.id.toString() !== params.programId,
                      }}
                    >
                      <ProgramItem program={program} index={i()} />
                    </li>
                  )}
                </For>
              </ul>
            </Match>
          </Switch>
          <div class="flex flex-row items-center overflow-visible">
            <AddProgram />
            <RefreshButton
              isFetching={isFetching()}
              onClick={() => programsQuery.refetch()}
              class="secondary-button ml-2"
            />
            <A href="/" class="text-button ml-2">
              Home
            </A>
          </div>
        </div>
      </div>
      <div class="h-full col-span-2 overflow-auto">
        <Outlet />
      </div>
    </div>
  );
};
