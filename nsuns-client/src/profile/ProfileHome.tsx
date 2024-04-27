import { A, Outlet, useParams } from "@solidjs/router";
import { Component, For, Match, Show, Switch, createEffect } from "solid-js";
import { AddProgram, LoadingProgram, ProgramItem } from "./Program";
import { ProfileGreeting } from "./ProfileGreeting";
import { SHIMMER_DELAY_MS, createDelayedLatch } from "../hooks/createDelayedLatch";
import { RefreshButton } from "../components/RefreshButton";
import { useProgramsQuery } from "../hooks/queries/programs";
import { useNavigateToNewProgram } from "../hooks/navigation";
import { MovementList } from "../movements/MovementList";
import { StatsProvider } from "../stats/StatsProvider";
import { createQuery } from "@tanstack/solid-query";
import { getProfile, isNotFound } from "../api";

export const ProfileHome: Component = () => {
  const params = useParams<{ profileId: string; programId?: string }>();
  const navToNewProgram = useNavigateToNewProgram();
  const programsQuery = useProgramsQuery(() => params.profileId);

  const profileQuery = createQuery({
    queryKey: () => ["profiles", params.profileId],
    queryFn: () => getProfile(params.profileId),
    enabled: !!params.profileId,
  });

  createEffect(() => {
    if (profileQuery.isSuccess && programsQuery.isSuccess && programsQuery.data?.length === 0) {
      navToNewProgram();
    }
  });

  const isFetching = createDelayedLatch(() => programsQuery.isFetching, SHIMMER_DELAY_MS);

  return (
    <div class="flex flex-col 2xl:h-full 2xl:grid 2xl:grid-rows-1 2xl:grid-cols-8 2xl:overflow-hidden">
      <div class="2xl:h-full py-12 px-24 2xl:overflow-auto 2xl:col-span-3">
        <div class="w-full mb-4">
          <div class="mb-4">
            <ProfileGreeting id={params.profileId} />
          </div>
          <Show when={!isNotFound(profileQuery.error)}>
            <h2 class="text-xl">Your Programs</h2>
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
                    <div class="mb-2">Failed to fetch programs: {`${programsQuery.error}`}</div>
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
                            "border-blue-500": program.id.toString() === params.programId,
                            "border-gray-600": program.id.toString() !== params.programId,
                          }}
                        >
                          <ProgramItem
                            program={program}
                            index={i()}
                            isActive={program.id.toString() === params.programId}
                          />
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
                  onClick={() => void programsQuery.refetch()}
                  class="secondary-button ml-2"
                />
                <A
                  href="/"
                  class="text-button ml-2"
                >
                  Switch Profile
                </A>
              </div>
            </div>
          </Show>
        </div>
        <StatsProvider profileId={params.profileId}>
          <MovementList />
        </StatsProvider>
      </div>
      <div class="2xl:h-full 2xl:col-span-5 2xl:overflow-auto">
        <Outlet />
      </div>
    </div>
  );
};
