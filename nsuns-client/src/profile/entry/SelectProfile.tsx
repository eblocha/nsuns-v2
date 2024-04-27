import { Component, For, Match, Switch } from "solid-js";
import { AddProfileCard, LoadingProfileCard, ProfileCard } from "./ProfileCard";
import { SHIMMER_DELAY_MS, createDelayedLatch } from "../../hooks/createDelayedLatch";
import { RefreshButton } from "../../components/RefreshButton";
import { createProfileQuery } from "../../hooks/queries/profiles";
import { LogoutButton } from "../../login/LogoutButton";

export const SelectProfile: Component = () => {
  const query = createProfileQuery();

  const isFetching = createDelayedLatch(() => query.isFetching, SHIMMER_DELAY_MS);

  return (
    <div class="h-full w-full overflow-hidden p-10 flex flex-col items-center justify-center gap-8">
      <h2 class="text-lg">Select a profile</h2>
      <Switch>
        <Match when={query.isLoading}>
          <ul class="flex flex-row items-center gap-4">
            <For each={[1, 2]}>
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
          <ul class="flex flex-row items-center gap-4">
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
      <div class="flex flex-col gap-4">
        <div class="flex flex-row items-center justify-center gap-2">
          <RefreshButton
            onClick={() => void query.refetch()}
            isFetching={isFetching()}
            class="secondary-button"
          />
        </div>
        <LogoutButton>Log Out</LogoutButton>
      </div>
    </div>
  );
};
