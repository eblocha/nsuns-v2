import { createFormControl, createFormGroup } from "solid-forms";
import { Component, Show, createSignal } from "solid-js";
import { TextInput } from "../forms/TextInput";
import { A, useNavigate } from "@solidjs/router";
import { createUser } from "../api";
import styles from "./CreateUser.module.css";

export const CreateUser: Component = () => {
  const [submitError, setSubmitError] = createSignal<unknown>(null);

  const navigator = useNavigate();

  const group = createFormGroup({
    username: createFormControl("", {
      required: true,
    }),
    name: createFormControl<string>(""),
  });

  const onSubmit = async () => {
    if (group.isSubmitted || !group.isValid) return;

    setSubmitError(null);
    group.markSubmitted(true);
    try {
      const user = await createUser({
        username: group.value.username ?? "user",
        name: group.value.name || null,
      });
      navigator(`/users/${user.id}`);
    } catch (e) {
      setSubmitError(e);
    } finally {
      group.markSubmitted(false);
    }
  };

  const resetForm = () => {
    for (const control of Object.values(group.controls)) {
      control.markDirty(false);
      control.markTouched(false);
      control.setValue("");
    }
  }

  return (
    <div class="w-full h-full flex flex-col items-center justify-center">
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          await onSubmit();
        }}
        class="mx-3 grid gap-y-2 gap-x-4"
        classList={{ [styles.form]: true }}
      >
        <h2 class="col-span-2 text-lg mb-4">Create User</h2>
        <label for="username" class="text-right">
          <span class="text-red-500">*</span>Username
        </label>
        <TextInput
          control={group.controls.username}
          class="ml-3 p-1 rounded"
          name="username"
        />
        <label for="name" class="text-right">
          Name
        </label>
        <TextInput
          control={group.controls.name}
          class="ml-3 p-1 rounded"
          name="name"
        />
        <div class="col-span-2">
          <div class="float-right flex flex-row items-center justify-end w-full">
            <A
              href="/"
              class="bg-gray-300 p-2 rounded hover:bg-gray-400 text-center mr-2"
            >
              Home
            </A>
            <button
              type="button"
              onClick={resetForm}
              class="bg-gray-300 p-2 rounded hover:bg-gray-400 text-center mr-2 disabled:text-gray-600 disabled:bg-gray-200"
              disabled={!group.isDirty || group.isSubmitted}
            >
              Reset
            </button>
            <button
              class="bg-blue-500 text-white p-2 rounded hover:bg-blue-600 disabled:bg-blue-300"
              disabled={!group.isValid || group.isSubmitted}
            >
              <Show when={!group.isSubmitted} fallback={<>Creating...</>}>
                Create User
              </Show>
            </button>
          </div>
          <Show when={submitError() !== null}>{`${submitError()}`}</Show>
        </div>
      </form>
    </div>
  );
};
