import { A, Outlet, useParams } from "@solidjs/router";
import { Component, For, Match, Show, Switch, createSignal } from "solid-js";
import { LoadingProgram, ProgramItem } from "./Program";
import { ProfileGreeting } from "./ProfileGreeting";
import { createMinimumAsyncDelay } from "../hooks/asymmetricDelay";
import { RefreshButton } from "../components/RefreshButton";
import { useProgramsQuery } from "../hooks/queries/programs";
import { MovementList } from "../movements/MovementList";
import { StatsProvider } from "../stats/StatsProvider";
import { createQuery } from "@tanstack/solid-query";
import { getProfile, isNotFound } from "../api";
import { Plus } from "../icons/Plus";
import { NewProgram } from "../program/NewProgram";

export const ProfileHome: Component = () => {
  const params = useParams<{ profileId: string; programId?: string }>();
  const programsQuery = useProgramsQuery(() => params.profileId);
  const [isAddingProgram, setIsAddingProgram] = createSignal(false);

  const profileQuery = createQuery({
    queryKey: () => ["profiles", params.profileId],
    queryFn: () => getProfile(params.profileId),
    enabled: !!params.profileId,
  });

  const isFetching = createMinimumAsyncDelay(() => programsQuery.isFetching);

  return (
    <div class="flex flex-col 2xl:h-full 2xl:grid 2xl:grid-rows-1 2xl:grid-cols-8 2xl:overflow-hidden">
      <div class="2xl:h-full py-12 px-24 2xl:overflow-auto 2xl:col-span-3">
        <div class="w-full mb-4">
          <div class="mb-4">
            <ProfileGreeting id={params.profileId} />
          </div>
          <Show when={!isNotFound(profileQuery.error)}>
            <h2 class="text-xl mb-2">Your Programs</h2>
            <div class="flex flex-col justify-center gap-4">
              <Switch>
                <Match when={programsQuery.isLoading}>
                  <ul class="w-full">
                    <For each={[1, 2, 3]}>
                      {() => (
                        <li class="mt-2">
                          <LoadingProgram />
                        </li>
                      )}
                    </For>
                  </ul>
                </Match>
                <Match when={programsQuery.isError}>
                  <div class="flex flex-col items-center justify-center">
                    Failed to fetch programs: {`${programsQuery.error}`}
                  </div>
                </Match>
                <Match when={programsQuery.isSuccess}>
                  <ul class="w-full">
                    <For each={programsQuery.data}>
                      {(program, i) => (
                        <li
                          class="rounded border mt-2"
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
              <Show when={isAddingProgram()}>
                <NewProgram close={() => setIsAddingProgram(false)} />
              </Show>
              <div class="flex flex-row items-center overflow-visible gap-2">
                <button
                  class="text-button-outline flex flex-row items-center justify-start gap-2"
                  onClick={() => setIsAddingProgram(true)}
                >
                  <Plus /> Add Program
                </button>
                <RefreshButton
                  isFetching={isFetching()}
                  onClick={() => void programsQuery.refetch()}
                  class="secondary-button"
                />
                <A
                  href="/"
                  class="text-button"
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
