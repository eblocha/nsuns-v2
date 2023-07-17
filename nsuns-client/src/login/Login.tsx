import { Component, For, Match, Show, Switch, createResource } from "solid-js";
import { getUsers } from "../api";
import { AddUserCard, LoadingUserCard, UserCard } from "./UserCard";
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
      <ul class="my-8 flex flex-row items-center">
        <For each={[1, 2, 3]}>
          {() => (
            <li>
              <LoadingUserCard />
            </li>
          )}
        </For>
      </ul>
    </Show>
  );
};

export const Login: Component = () => {
  const query = createQuery(() => ["users"], getUsers);

  return (
    <div class="h-full w-full overflow-hidden p-10 flex flex-col items-center justify-center">
      <h2 class="text-lg">Select a user</h2>
      <Switch>
        <Match when={query.isLoading}>
          <ul class="my-8 flex flex-row items-center">
            <For each={[1, 2, 3]}>
              {() => (
                <li>
                  <LoadingUserCard />
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
          <ul class="my-8 flex flex-row items-center">
            <For each={query.data}>
              {(user) => (
                <li>
                  <UserCard {...user} />
                </li>
              )}
            </For>
            <li>
              <AddUserCard />
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
  );
};
