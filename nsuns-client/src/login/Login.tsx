import { Component, For, Show, createResource } from "solid-js";
import { getUsers } from "../api";
import { LoadingUserCard, UserCard } from "./UserCard";
import { A } from "@solidjs/router";

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

const NoUsers: Component = () => {
  return <p class="h-44 flex flex-col items-center justify-center">No users.</p>;
};

export const Login: Component = () => {
  const [data, { refetch }] = createResource(getUsers);

  return (
    <div class="h-full w-full overflow-hidden p-10 flex flex-col items-center justify-center">
      <Show
        when={data.state === "ready"}
        fallback={
          <LoadingOrError isLoading={data.loading} error={data.error} />
        }
      >
        <ul class="my-8 flex flex-row items-center">
          <For each={data.latest} fallback={<NoUsers />}>
            {(user) => (
              <li>
                <UserCard {...user} />
              </li>
            )}
          </For>
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
        <A
          href="/create-user"
          class="p-3 rounded bg-blue-500 hover:bg-blue-600 active:bg-blue-600 text-center text-white"
        >
          Create User
        </A>
      </div>
    </div>
  );
};
