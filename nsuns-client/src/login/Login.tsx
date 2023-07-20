import { Component, For, Match, Switch } from "solid-js";
import { getProfiles } from "../api";
import { AddProfileCard, LoadingProfileCard, ProfileCard } from "./ProfileCard";
import { createQuery } from "@tanstack/solid-query";

export const Login: Component = () => {
  const query = createQuery(() => ["profiles"], getProfiles);

  return (
    <div class="h-full w-full overflow-hidden p-10 flex flex-col items-center justify-center">
      <h2 class="text-lg">Select a profile</h2>
      <Switch>
        <Match when={query.isLoading}>
          <ul class="my-8 flex flex-row items-center">
            <For each={[1, 2, 3]}>
              {() => (
                <li>
                  <LoadingProfileCard />
                </li>
              )}
            </For>
          </ul>
        </Match>
        <Match when={query.isError}>
          <div class="flex flex-col items-center justify-center my-10">
            <div class="mb-2">Error fetching profiles: {`${query.error}`}</div>
          </div>
        </Match>
        <Match when={query.isSuccess}>
          <ul class="my-8 flex flex-row items-center">
            <For each={query.data}>
              {(profile) => (
                <li>
                  <ProfileCard {...profile} />
                </li>
              )}
            </For>
            <li>
              <AddProfileCard />
            </li>
          </ul>
        </Match>
      </Switch>
      <div class="flex flex-row items-center justify-center">
        <button
          onClick={() => query.refetch()}
          disabled={query.isFetching}
          class="secondary-button"
        >
          Refresh
        </button>
      </div>
    </div>
  );
};
