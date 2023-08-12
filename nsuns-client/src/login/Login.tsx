import { Component, For, Match, Switch } from "solid-js";
import { AddProfileCard, LoadingProfileCard, ProfileCard } from "./ProfileCard";
import { createDelayedLatch } from "../hooks/createDelayedLatch";
import { RefreshButton } from "../components/RefreshButton";
import { createProfileQuery } from "../hooks/queries/profiles";

export const Login: Component = () => {
  const query = createProfileQuery();

  const isFetching = createDelayedLatch(() => query.isFetching, 200);

  return (
    <div class="h-full w-full overflow-hidden p-10 flex flex-col items-center justify-center">
      <h2 class="text-lg">Select a profile</h2>
      <Switch>
        <Match when={query.isLoading}>
          <ul class="my-8 flex flex-row items-center gap-4">
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
          <ul class="my-8 flex flex-row items-center gap-4">
            <For each={query.data}>
              {(profile) => (
                <li
                  class="rounded"
                  classList={{
                    shimmer: isFetching(),
                  }}
                >
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
        <RefreshButton
          onClick={() => void query.refetch()}
          isFetching={isFetching()}
          class="secondary-button"
        />
      </div>
    </div>
  );
};
