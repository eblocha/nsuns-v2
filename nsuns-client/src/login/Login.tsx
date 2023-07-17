import { Component, For, Show, createResource } from "solid-js";
import { getUsers } from "../api";
import { AddUserCard, LoadingUserCard, UserCard } from "./UserCard";

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
  const [data, { refetch }] = createResource(getUsers);

  return (
    <div class="h-full w-full overflow-hidden p-10 flex flex-col items-center justify-center">
      <h2 class="text-lg">Select a user</h2>
      <Show
        when={data.state === "ready"}
        fallback={
          <LoadingOrError isLoading={data.loading} error={data.error} />
        }
      >
        <ul class="my-8 flex flex-row items-center">
          <For each={data.latest}>
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
      </Show>
      <div class="flex flex-row items-center justify-center">
        <button
          onClick={refetch}
          disabled={data.loading}
          class="p-3 rounded bg-gray-300 hover:bg-gray-400 active:bg-gray-500 mr-2"
        >
          Refresh
        </button>
      </div>
    </div>
  );
};
