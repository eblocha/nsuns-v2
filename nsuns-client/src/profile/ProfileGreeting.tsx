import { Component, Match, Show, Switch, createRenderEffect, createSignal } from "solid-js";
import { Profile, getProfile, isNotFound } from "../api";
import { createQuery } from "@tanstack/solid-query";
import { Input } from "../forms/Input";
import { createControl, required } from "../hooks/forms";
import { createUpdateProfileMutation } from "../hooks/queries/profiles";
import { Trash } from "../icons/Trash";
import { DeleteProfile } from "./DeleteProfile";
import { A } from "@solidjs/router";
import { Warning } from "../icons/Warning";
import { LogoutButton } from "../login/LogoutButton";

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
      <Input
        control={name}
        class="ghost-input flex-grow"
        required={true}
      />
      <Show when={name.isChanged()}>
        <button
          class="primary-button text-base"
          disabled={disableSubmit()}
        >
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
  const [showDeleteModal, setShowDeleteModal] = createSignal(false);
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
          <button
            class="text-button text-base text-gray-600 hover:text-red-500 focus:text-red-500 hover:transition-colors focus:transition-colors"
            onClick={(e) => {
              e.preventDefault();
              setShowDeleteModal(true);
            }}
          >
            <Trash />
          </button>
          <LogoutButton />
        </h2>
        <DeleteProfile
          show={showDeleteModal()}
          close={() => setShowDeleteModal(false)}
          profile={query.data!}
        />
      </Match>
      <Match when={query.isError && isNotFound(query.error)}>
        <div class="flex flex-row items-center gap-4">
          <div class="flex flex-row items-center gap-2">
            <Warning class="text-red-500" />
            <h2 class="text-lg">Profile Not Found</h2>
          </div>
          <A
            href="/"
            class="secondary-button"
          >
            Switch Profile
          </A>
          <LogoutButton />
        </div>
      </Match>
    </Switch>
  );
};
