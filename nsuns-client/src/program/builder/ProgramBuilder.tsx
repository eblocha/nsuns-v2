import { useParams } from "@solidjs/router";
import { createQuery } from "@tanstack/solid-query";
import { Component, Match, Switch } from "solid-js";
import { getProgramSummary } from "../../api";
import { MovementList } from "../../movements/MovementList";

const Loading: Component = () => {
  return (
    <div class="grid grid-rows-6 gap-10 w-full h-full">
      <div class="w-full h-full grid grid-cols-2 gap-4">
        <div class="shimmer rounded"></div>
        <div class="shimmer rounded"></div>
      </div>
      <div class="shimmer rounded"></div>
      <div class="shimmer rounded"></div>
      <div class="shimmer rounded"></div>
      <div class="shimmer rounded"></div>
      <div class="shimmer rounded"></div>
    </div>
  );
};

const Error: Component<{ message: string }> = (props) => {
  return (
    <div class="w-full h-full flex flex-col items-center justify-center">
      <div>Error fetching program details:</div>
      <div>{props.message}</div>
    </div>
  );
};

export const ProgramBuilder: Component = () => {
  const params = useParams<{ profileId: string; programId: string }>();

  const query = createQuery({
    queryKey: () => ["programs", params.programId],
    queryFn: () => getProgramSummary(params.programId),
    enabled: !!params.programId,
  });

  return (
    <div class="w-full h-full overflow-hidden border-l border-gray-700 grid grid-cols-4 gap-5 p-5">
      <div class="col-span-3 flex flex-col">
        <h2 class="mb-4 text-xl">Program Details</h2>
        <div class="flex-grow">
          <Switch>
            <Match when={query.isLoading}>
              <Loading />
            </Match>
            <Match when={query.isError}>
              <Error message={`${query.error}`} />
            </Match>
            <Match when={query.isSuccess}>
              <div></div>
            </Match>
          </Switch>
        </div>
      </div>
      <div>
        <MovementList />
      </div>
    </div>
  );
};
