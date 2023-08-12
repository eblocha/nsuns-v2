import { Component, Show, createMemo } from "solid-js";
import { Control, ControlGroup } from "../../hooks/forms";
import { Select, SelectOption } from "../../forms/Select";
import { Checkbox } from "../../forms/Checkbox";
import { TextArea } from "../../forms/TextArea";
import { CreateMutationResult } from "@tanstack/solid-query";
import { CreateProgramSet, Movement, UpdateProgramSet } from "../../api";
import { Spinner } from "../../icons/Spinner";
import { Input } from "../../forms/Input";
import { Warning } from "../../icons/Warning";
import { displayError } from "../../util/errors";
import { Day } from "../../util/days";
import { DeleteSetMeta } from "../../hooks/queries/sets";

export type SetFormControls = ControlGroup<{
  movementId: Control<string>;
  reps: Control<string>;
  repsIsMinimum: Control<boolean>;
  description: Control<string>;
  amount: Control<string>;
  maxMovementId: Control<string>;
}>;

export const SetForm: Component<{
  group: SetFormControls;
  onSubmit?: () => void;
  onClose: () => void;
  title: string;
  mutationCreate?: CreateMutationResult<
    unknown,
    unknown,
    CreateProgramSet,
    unknown
  >;
  mutationUpdate?: CreateMutationResult<
    unknown,
    unknown,
    UpdateProgramSet & { day: Day; programId: string },
    unknown
  >;
  mutationDelete?: CreateMutationResult<
    unknown,
    unknown,
    { id: string; meta: DeleteSetMeta },
    unknown
  >;
  id?: string;
  programId: string;
  dayIndex: Day;
  movements?: Movement[];
}> = (props) => {
  const isLoading = () =>
    props.mutationCreate?.isLoading || !!props.mutationUpdate?.isLoading;

  const isDeleting = () => !!props.mutationDelete?.isLoading;

  const isError = () =>
    props.mutationCreate?.isError || !!props.mutationUpdate?.isError;

  const error = () =>
    props.mutationCreate?.error || props.mutationUpdate?.error;

  const disableSubmit = () =>
    isLoading() ||
    props.group.hasErrors() ||
    (props.mutationUpdate && !props.group.dirty());

  const onSubmit = () => {
    if (disableSubmit()) return;
    const value = props.group.value();

    if (!value.amount || !value.movementId) return;

    const base = {
      amount: parseFloat(value.amount),
      day: props.dayIndex,
      description: value.description || null,
      movementId: value.movementId,
      percentageOfMax: value.maxMovementId ? value.maxMovementId : null,
      programId: props.programId,
      reps: value.reps ? parseInt(value.reps) : null,
      repsIsMinimum: value.repsIsMinimum,
    };

    if (props.mutationCreate) {
      props.mutationCreate.mutate(base);
    } else if (props.id && props.mutationUpdate) {
      props.mutationUpdate.mutate({ ...base, id: props.id });
    }
    props.onSubmit?.();
  };

  const movementOptions = createMemo<SelectOption[] | undefined>(() =>
    props.movements?.map((movement) => ({
      value: movement.id.toString(),
      name: movement.name,
    }))
  );

  const onDelete = () => {
    !isDeleting() &&
      props.id &&
      props.mutationDelete?.mutate({
        id: props.id,
        meta: {
          day: props.dayIndex,
          programId: props.programId,
        },
      });
  };

  return (
    <form
      class="grid grid-cols-2 gap-y-4 gap-x-2"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <h3 class="col-span-2 text-lg mb-2">{props.title}</h3>
      <label>
        <span class="text-red-500">*</span>Movement:
        <Select
          class="input w-full"
          control={props.group.controls.movementId}
          options={movementOptions()}
          required={true}
        />
      </label>
      <div>
        <label class="w-full" for="reps">
          Reps:
        </label>
        <div class="grid grid-cols-2 gap-2">
          <Input
            control={props.group.controls.reps}
            class="input w-full"
            type="number"
            name="reps"
            min={0}
          />
          <label class="flex flex-row items-center">
            <Checkbox
              control={props.group.controls.repsIsMinimum}
              class="mr-3"
            />
            Is Minimum?
          </label>
        </div>
      </div>

      <div class="col-span-2 grid grid-cols-2 gap-2">
        <label>
          <span class="text-red-500">*</span>
          <Show
            when={!props.group.controls.maxMovementId.value()}
            fallback={"Percent:"}
          >
            Weight:
          </Show>
          <Input
            control={props.group.controls.amount}
            class="input w-full"
            type="number"
            min={0}
            required={true}
          />
        </label>
        <label>
          Percent of Max In:
          <Select
            control={props.group.controls.maxMovementId}
            class="input w-full"
            options={movementOptions()}
            emptyOption=""
          />
        </label>
      </div>
      <label class="col-span-2">
        Description:
        <TextArea
          control={props.group.controls.description}
          class="input w-full"
        />
      </label>

      <div class="col-span-2 flex flex-row items-center justify-end gap-2">
        <Show when={props.mutationDelete}>
          <button
            type="button"
            class="danger-button w-16 flex flex-row items-center justify-center h-full mr-auto"
            onClick={onDelete}
          >
            <Show
              when={!isDeleting()}
              fallback={<Spinner class="animate-spin" />}
            >
              Delete
            </Show>
          </button>
        </Show>
        <button type="button" class="secondary-button" onClick={props.onClose}>
          Cancel
        </button>
        <button
          type="submit"
          class="primary-button w-20 flex flex-row items-center justify-center h-full"
          disabled={disableSubmit()}
        >
          <Show when={!isLoading()} fallback={<Spinner class="animate-spin" />}>
            Confirm
          </Show>
        </button>
      </div>
      <Show when={isError()}>
        <div class="col-span-2 flex flex-row items-center justify-end gap-4 mt-2">
          <span>
            <Warning class="text-red-500" />
          </span>
          {displayError(error(), "save set")}
        </div>
      </Show>
    </form>
  );
};
