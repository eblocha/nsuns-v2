import { Component, Show } from "solid-js";
import { createControl, createControlGroup, required } from "../hooks/forms";
import { displayError } from "../util/errors";
import { Warning } from "../icons/Warning";
import { Spinner } from "../icons/Spinner";
import { A } from "@solidjs/router";
import { ErrorMessages } from "../forms/ErrorMessages";
import { Input } from "../forms/Input";
import styles from "./LoginForm.module.css";
import { createMutation } from "@tanstack/solid-query";
import { login } from "../api/auth";
import { useNavigateToProfileHome } from "../hooks/navigation";

export const LoginForm: Component = () => {
  const navigate = useNavigateToProfileHome();

  const form = createControlGroup({
    username: createControl<string>("", { validators: [required()] }),
    password: createControl<string>("", { validators: [required()] }),
  });

  const mutation = createMutation({
    mutationFn: login,
    onSuccess: () => {
      navigate();
    },
  });

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        if (form.hasErrors()) {
          return;
        }

        mutation.mutate({
          username: form.value().username,
          password: form.value().password,
        });
      }}
      class="grid gap-4 w-80"
    >
      <h2 class="text-lg">Log In</h2>
      <div class={`grid grid-cols-2 gap-2 ${styles.controls}`}>
        <label
          for="name"
          class="flex flex-row items-center"
        >
          <span class="text-red-500">*</span>Username
        </label>
        <div class="flex flex-col items-end flex-grow">
          <Input
            control={form.controls.username}
            class="ml-3 input w-full"
            name="name"
            required={true}
          />
        </div>
        <div class="col-span-2 flex flex-row justify-end">
          <ErrorMessages control={form.controls.username} />
        </div>
        <label
          for="password"
          class="flex flex-row items-center"
        >
          <span class="text-red-500">*</span>Password
        </label>
        <div class="flex flex-col items-end flex-grow">
          <Input
            control={form.controls.password}
            class="ml-3 input w-full"
            name="password"
            type="password"
            required={true}
          />
        </div>
        <div class="col-span-2 flex flex-row justify-end">
          <ErrorMessages control={form.controls.password} />
        </div>
      </div>

      <div class="float-right flex flex-row items-center justify-end w-full gap-2">
        <A
          href="/"
          class="text-button text-center"
        >
          Home
        </A>
        <button
          type="button"
          onClick={() => form.reset()}
          class="secondary-button"
          disabled={!form.dirty()}
        >
          Reset
        </button>
        <button
          type="submit"
          class="primary-button"
          disabled={mutation.isLoading}
        >
          <Show
            when={!mutation.isLoading}
            fallback={<Spinner class="animate-spin my-1" />}
          >
            Log In
          </Show>
        </button>
      </div>
      <Show when={mutation.isError}>
        <div class="w-full flex flex-row items-center justify-end gap-4">
          <span>
            <Warning class="text-red-500" />
          </span>
          {displayError(mutation.error, "log in")}
        </div>
      </Show>
    </form>
  );
};
