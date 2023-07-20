import { A, Outlet, useNavigate, useParams } from "@solidjs/router";
import { Component, For, Match, Switch, createEffect } from "solid-js";
import { getProfilePrograms } from "../api/program";
import { AddProgram, LoadingProgram, ProgramItem } from "./Program";
import { createQuery } from "@tanstack/solid-query";
import { ProfileGreeting } from "./ProfileGreeting";

export const ProfileHome: Component = () => {
  const params = useParams<{ profileId: string; programId?: string }>();
  const navigate = useNavigate();
  const programsQuery = createQuery({
    queryKey: () => ["programs", params.profileId],
    queryFn: () => getProfilePrograms(params.profileId),
    enabled: !!params.profileId,
  });

  createEffect(() => {
    if (programsQuery.isSuccess && programsQuery.data?.length === 0) {
      navigate("program/new");
    }
  });

  return (
    <div class="h-full grid grid-cols-3 gap-10">
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
                      class="rounded border"
                      classList={{
                        shimmer: programsQuery.isFetching,
                        "border-blue-500": program.id.toString() === params.programId,
                        "border-gray-600": program.id.toString() !== params.programId,
                      }}
                    >
                      <ProgramItem program={program} index={i()} />
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
