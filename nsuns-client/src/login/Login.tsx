import { Component, Show } from "solid-js";
import { Input } from "../forms/Input";
import { createControl, required } from "../hooks/forms";
import { ErrorMessages } from "../forms/ErrorMessages";
import { A } from "@solidjs/router";
import { Spinner } from "../icons/Spinner";
import { Warning } from "../icons/Warning";
import { displayError } from "../util/errors";

export const Login: Component = () => {
  const username = createControl<string>("", { validators: [required()] });
  const password = createControl<string>("", { validators: [required()] });

  const onSubmit = () => {};

  return (
    <div class="w-full h-full flex flex-col items-center justify-center">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          onSubmit();
        }}
        class="flex flex-col w-80 gap-4"
      >
        <h2 class="text-lg">Log In</h2>
        <label
          for="name"
          class="flex flex-row items-center gap-2"
        >
          <span class="text-red-500">*</span>username
          <div class="flex flex-col items-end flex-grow">
            <Input
              control={username}
              class="ml-3 input w-full"
              name="name"
              required={true}
            />
            <ErrorMessages control={username} />
          </div>
        </label>
        <label
          for="password"
          class="flex flex-row items-center gap-2"
        >
          <span class="text-red-500">*</span>password
          <div class="flex flex-col items-end flex-grow">
            <Input
              control={password}
              class="ml-3 input w-full"
              name="password"
              type="password"
              required={true}
            />
            <ErrorMessages control={password} />
          </div>
        </label>

        <div class="float-right flex flex-row items-center justify-end w-full gap-2">
          <A
            href="/"
            class="text-button text-center"
          >
            Home
          </A>
          <button
            type="button"
            onClick={() => username.reset()}
            class="secondary-button"
            disabled={!username.dirty()}
          >
            Reset
          </button>
          <button
            type="submit"
            class="bg-blue-500 text-white p-2 rounded hover:bg-blue-600 disabled:bg-blue-400 w-32 flex flex-row items-center justify-center"
            disabled={false}
          >
            <Show
              when={true}
              fallback={<Spinner class="animate-spin my-1" />}
            >
              Log In
            </Show>
          </button>
        </div>
        <Show when={false}>
          <div class="w-full flex flex-row items-center justify-end gap-4">
            <span>
              <Warning class="text-red-500" />
            </span>
            {displayError("", "create profile")}
          </div>
        </Show>
      </form>
    </div>
  );
};
