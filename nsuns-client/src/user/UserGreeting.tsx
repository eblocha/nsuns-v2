import { Component, Match, Switch } from "solid-js";
import { getUser } from "../api";
import { createQuery } from "@tanstack/solid-query";

const Pending: Component = () => {
  return <h2 class="h-10 w-60 shimmer"></h2>;
};

export const UserGreeting: Component<{ id: string }> = (props) => {
  const userQuery = createQuery({
    queryKey: () => ["users", props.id],
    queryFn: () => getUser(props.id),
    enabled: !!props.id,
  });

  return (
    <Switch>
      <Match when={userQuery.isLoading}>
        <Pending />
      </Match>
      <Match when={userQuery.isSuccess}>
        <h2 class="text-2xl h-10">
          Welcome, {userQuery.data?.name || userQuery.data?.username}
        </h2>
      </Match>
    </Switch>
  );
};
