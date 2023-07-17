import { useParams } from "@solidjs/router";
import { Component, For, Show, createResource } from "solid-js";
import { getUserPrograms } from "../api/program";
import { AddProgram, LoadingProgram, ProgramItem } from "./Program";

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
      <ul class="my-8 w-full">
        <For each={[1, 2, 3]}>
          {() => (
            <li>
              <LoadingProgram />
            </li>
          )}
        </For>
      </ul>
    </Show>
  );
};

export const UserHome: Component = () => {
  const params = useParams<{id: string}>();
  const [data, { refetch }] = createResource(params.id, getUserPrograms);

  return (
    <div class="h-full w-full p-60">
      <h2 class="text-lg">Your Programs</h2>
      <div class="flex flex-col items-center justify-center">
        <Show
          when={data.state === "ready"}
          fallback={
            <LoadingOrError isLoading={data.loading} error={data.error} />
          }
        >
          <ul class="my-8 w-full">
            <For each={data.latest?.all}>
              {(program, i) => (
                <li>
                  <ProgramItem
                    program={program}
                    index={i()}
                    isDefault={program.id === data.latest?.default?.id}
                  />
                </li>
              )}
            </For>
            <li>
              <AddProgram />
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
    </div>
  );
};
