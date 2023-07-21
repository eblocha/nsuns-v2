import { createFormControl, createFormGroup } from "solid-forms";
import { Component, Show, createRenderEffect, on } from "solid-js";
import { TextInput } from "../../forms/TextInput";
import { Program } from "../../api";
import { hasErrors } from "../../forms/errors";
import { Spinner } from "../../icons/Spinner";
import { createDelayedLatch } from "../../hooks/createDelayedLatch";
import { useUpdateProgram } from "../../hooks/queries/programs";

export const ProgramDetails: Component<Program> = (props) => {
  const group = createFormGroup({
    name: createFormControl(props.name, { required: true }),
  });

  const mutation = useUpdateProgram();

  const anyErrors = () => {
    return !Object.values(group.controls).every(
      (control) => !hasErrors(control.errors)
    );
  };

  const onSubmit = async () => {
    if (mutation.isLoading || anyErrors()) return;

    mutation.mutate({
      id: props.id,
      name: group.value.name!,
    });
  };

  const reset = (name: string) => {
    group.setValue({ name });
    for (const control of Object.values(group.controls)) {
      control.markDirty(false);
      control.markTouched(false);
    }
  };

  createRenderEffect(
    on(
      () => props.name,
      () => {
        reset(props.name ?? "");
      }
    )
  );

  const isLoading = createDelayedLatch(() => mutation.isLoading, 200);

  return (
    <form
      class="flex flex-row items-center justify-between mb-2"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <TextInput control={group.controls.name} class="ghost-input" />
      <button
        class="primary-button ml-4 w-16 flex flex-row items-center justify-center"
        disabled={isLoading() || anyErrors() || !group.isDirty}
      >
        <Show
          when={!isLoading()}
          fallback={<Spinner class="my-1 animate-spin" />}
        >
          Save
        </Show>
      </button>
    </form>
  );
};
