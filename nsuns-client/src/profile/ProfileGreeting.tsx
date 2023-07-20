import { Component, Match, Switch } from "solid-js";
import { getProfile } from "../api";
import { createQuery } from "@tanstack/solid-query";

const Pending: Component = () => {
  return <h2 class="h-10 w-60 shimmer"></h2>;
};

export const ProfileGreeting: Component<{ id: string }> = (props) => {
  const query = createQuery({
    queryKey: () => ["profiles", props.id],
    queryFn: () => getProfile(props.id),
    enabled: !!props.id,
  });

  return (
    <Switch>
      <Match when={query.isLoading}>
        <Pending />
      </Match>
      <Match when={query.isSuccess}>
        <h2 class="text-2xl h-10">
          Welcome, {query.data?.name}
        </h2>
      </Match>
    </Switch>
  );
};
