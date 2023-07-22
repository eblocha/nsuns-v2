import { Component, Show } from "solid-js";
import styles from "./MovementForm.module.css";
import { Input } from "../forms/Input";
import { Control, ControlGroup } from "../hooks/forms";
import { ErrorMessages } from "../forms/ErrorMessages";
import { TextArea } from "../forms/TextArea";
import { CreateMutationResult } from "@tanstack/solid-query";
import { CreateMovement, Movement } from "../api";
import { Spinner } from "../icons/Spinner";
import { Warning } from "../icons/Warning";
import { displayError } from "../util/errors";

export type MovementFormControls = ControlGroup<{
  name: Control<string>;
  description: Control<string>;
}>;

export const MovementForm: Component<{
  group: MovementFormControls;
  mutationCreate?: CreateMutationResult<
    Movement,
    unknown,
    CreateMovement,
    unknown
  >;
  mutationUpdate?: CreateMutationResult<Movement, unknown, Movement, unknown>;
  id?: number;
  confirmText: string;
  onSubmit?: () => void;
  onClose: () => void;
}> = (props) => {
  const isLoading = () =>
    props.mutationCreate?.isLoading || !!props.mutationUpdate?.isLoading;

  const isError = () =>
    props.mutationCreate?.isError || !!props.mutationUpdate?.isError;
  const error = () =>
    props.mutationCreate?.error || props.mutationUpdate?.error;

  const onSubmit = () => {
    if (props.group.hasErrors() || isLoading()) return;
    const value = props.group.value();

    if (!value.name) return;

    const base = {
      name: value.name,
      description: value.description || null,
    };

    if (props.mutationCreate) {
      props.mutationCreate.mutate(base);
    } else if (props.id && props.mutationUpdate) {
      props.mutationUpdate.mutate({ ...base, id: props.id });
    }
    props.onSubmit?.();
  };

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
      classList={{ [styles.form]: true }}
    >
      <label for="movement-name">
        <span class="text-red-500">*</span>Name
      </label>
      <div class="flex flex-col items-end">
        <Input
          control={props.group.controls.name}
          class="input w-full my-2"
          name="movement-name"
          required={true}
        />
        <ErrorMessages control={props.group.controls.name} />
      </div>
      <label for="movement-description">Description</label>
      <div class="flex flex-col items-end">
        <TextArea
          control={props.group.controls.description}
          class="input w-full my-2"
          name="movement-description"
        />
        <ErrorMessages control={props.group.controls.description} />
      </div>
      <div class="flex flex-row items-center mt-2">
        <button
          type="button"
          class="secondary-button mr-2"
          disabled={isLoading()}
          onClick={props.onClose}
        >
          Cancel
        </button>
        <button
          type="submit"
          class="primary-button w-40 flex flex-row items-center justify-center h-10"
          disabled={
            isLoading() || props.group.hasErrors() || !props.group.dirty()
          }
        >
          <Show when={!isLoading()} fallback={<Spinner class="animate-spin" />}>
            {props.confirmText}
          </Show>
        </button>
      </div>
      <Show when={isError()}>
        <div class="w-full flex flex-row items-center justify-end gap-4 mt-2">
          <span>
            <Warning class="text-red-500" />
          </span>
          {displayError(error(), "save movement")}
        </div>
      </Show>
    </form>
  );
};
