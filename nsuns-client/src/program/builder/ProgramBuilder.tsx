import { A, useParams } from "@solidjs/router";
import { Component, Match, Switch, createEffect, onCleanup } from "solid-js";
import { Days } from "./Days";
import { ProgramDetails } from "./ProgramDetails";
import { useProgramSummaryQuery } from "../../hooks/queries/sets";

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
      <div>Failed to fetch program details:</div>
      <div>{props.message}</div>
    </div>
  );
};

export const ProgramBuilder: Component = () => {
  const params = useParams<{ profileId: string; programId: string }>();

  const query = useProgramSummaryQuery(() => params.programId);

  let view: HTMLDivElement | undefined;

  createEffect(() => {
    const timeout = setTimeout(() => {
      view?.scrollIntoView({
        behavior: "smooth",
        block: "start",
      });
    }, 50);

    onCleanup(() => clearTimeout(timeout));
  });

  return (
    <div
      ref={view}
      class="w-full min-h-full overflow-visible 2xl:border-l border-gray-700 p-5 relative"
    >
      <div class="flex flex-col overflow-visible relative">
        <Switch>
          <Match when={query.isLoading}>
            <Loading />
          </Match>
          <Match when={query.isError}>
            <Error message={`${query.error}`} />
          </Match>
          <Match when={query.isSuccess}>
            <div class="mb-8 border-b border-gray-700 flex-shrink-0 flex flex-row gap-2">
              <div class="flex-grow">
                <ProgramDetails
                  program={query.data!.program}
                  profileId={params.profileId}
                />
              </div>
              <div class="flex-shrink-0">
                <A
                  href={`/profile/${params.profileId}`}
                  class="text-button"
                >
                  Close
                </A>
              </div>
            </div>
            <div class="flex-grow overflow-visible">
              <Days summary={query.data!} />
            </div>
          </Match>
        </Switch>
      </div>
    </div>
  );
};
