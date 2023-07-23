import { Component, Match, Show, Switch, createRenderEffect } from "solid-js";
import { Profile, getProfile } from "../api";
import { createQuery } from "@tanstack/solid-query";
import { Input } from "../forms/Input";
import { createControl, required } from "../hooks/forms";
import { createUpdateProfileMutation } from "../hooks/queries/profiles";

const EditProfileName: Component<{ profile: Profile }> = (props) => {
  const name = createControl(props.profile.name, { validators: [required()] });
  const reset = () => name.reset(props.profile.name);
  createRenderEffect(reset);

  const mutation = createUpdateProfileMutation({
    onSuccess: (profile) => {
      name.reset(profile.name);
    },
  });

  const disableSubmit = () => mutation.isLoading || name.hasErrors() || !name.isChanged();

  const onSubmit = () => {
    if (disableSubmit()) return;

    mutation.mutate({
      id: props.profile.id,
      name: name.value(),
    });
  };

  return (
    <form
      class="flex flex-row items-center gap-2 flex-grow"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <Input control={name} class="ghost-input flex-grow" required={true} />
      <Show when={name.isChanged()}>
        <button class="primary-button text-base" disabled={disableSubmit()}>
          Save
        </button>
      </Show>
    </form>
  );
};

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
        <h2 class="text-2xl h-10 flex flex-row items-center gap-2">
          Welcome, <EditProfileName profile={query.data!} />
        </h2>
      </Match>
    </Switch>
  );
};
