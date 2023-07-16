import { createFormControl, createFormGroup } from "solid-forms";
import { Component } from "solid-js";
import { TextInput } from "../forms/TextInput";
import { A, useNavigate } from "@solidjs/router";
import { createUser } from "../api";
import styles from "./CreateUser.module.css";

export const CreateUser: Component = () => {
  const navigator = useNavigate();

  const group = createFormGroup({
    username: createFormControl("", {
      required: true,
    }),
    name: createFormControl<string>(""),
  });

  const onSubmit = async () => {
    if (group.isSubmitted || !group.isValid) return;

    group.markSubmitted(true);
    const user = await createUser({
      username: group.value.username ?? "user",
      name: group.value.name || null,
    });
    navigator(`/users/${user.id}`);
  };

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
          <div class="float-right flex flex-row items-center">
            <A href="/" class="bg-gray-200 p-2 rounded hover:bg-gray-300 text-center mr-2">
              Home
            </A>
            <button class="bg-blue-500 text-white p-2 rounded hover:bg-blue-600">
              Create User
            </button>
          </div>
        </div>
      </form>
    </div>
  );
};
