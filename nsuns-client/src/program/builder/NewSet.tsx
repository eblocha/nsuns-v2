import { Component, Show, createEffect, createMemo } from "solid-js";
import { Day, Movement } from "../../api";
import { useCreateSet } from "../../hooks/queries/sets";
import { Spinner } from "../../icons/Spinner";
import { createControl, createControlGroup, required } from "../../hooks/forms";
import { Select, SelectOption } from "../../forms/Select";
import { Input } from "../../forms/Input";
import { TextArea } from "../../forms/TextArea";

export const NewSet: Component<{
  close: () => void;
  dayIndex: number;
  programId: number;
  movements?: Movement[];
}> = (props) => {
  const mutation = useCreateSet({
    onSuccess: () => {
      props.close();
    },
  });

  const group = createControlGroup({
    movementId: createControl<string>("", { validators: [required()] }),
    reps: createControl<string>(""),
    repsIsMinimum: createControl("true"),
    description: createControl<string>(""),
    amount: createControl<string>("0", { validators: [required()] }),
    maxMovementId: createControl<string>(""),
  });

  const onSubmit = () => {
    if (mutation.isLoading || group.hasErrors()) return;
    const value = group.value();

    if (!value.amount || !value.movementId) return;

    mutation.mutate({
      amount: parseFloat(value.amount),
      day: props.dayIndex as Day,
      description: value.description ?? null,
      movementId: parseInt(value.movementId),
      percentageOfMax: value.maxMovementId
        ? parseInt(value.maxMovementId)
        : null,
      programId: props.programId,
      reps: value.reps ? parseInt(value.reps) : null,
      repsIsMinimum: value.repsIsMinimum === "true",
    });
  };

  const movementOptions = createMemo<SelectOption[] | undefined>(() =>
    props.movements?.map((movement) => ({
      value: movement.id.toString(),
      name: movement.name,
    }))
  );

  return (
    <form
      class="grid grid-cols-2 gap-y-4 gap-x-2 border border-gray-700 p-4 rounded"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <h3 class="col-span-3 text-lg mb-2">Add Set</h3>
      <label>
        <span class="text-red-500">*</span>Movement:
        <Select
          class="input w-full"
          control={group.controls.movementId}
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
            control={group.controls.reps}
            class="input w-full"
            type="number"
            name="reps"
            min={0}
          />
          <label class="flex flex-row items-center">
            <Input
              control={group.controls.repsIsMinimum}
              class="mr-3"
              type="checkbox"
            />
            Is Minimum?
          </label>
        </div>
      </div>

      <div class="col-span-2 grid grid-cols-2 gap-2">
        <label>
          <span class="text-red-500">*</span>
          <Show
            when={!group.controls.maxMovementId.value()}
            fallback={"Percent:"}
          >
            Weight:
          </Show>
          <Input
            control={group.controls.amount}
            class="input w-full"
            type="number"
            min={0}
            required={true}
          />
        </label>
        <label>
          Percent of Max In:
          <Select
            control={group.controls.maxMovementId}
            class="input w-full"
            options={movementOptions()}
            emptyOption=""
          />
        </label>
      </div>
      <label class="col-span-2">
        Description:
        <TextArea control={group.controls.description} class="input w-full" />
      </label>

      <div class="col-span-2 flex flex-row items-center justify-end gap-2">
        <button type="button" class="secondary-button" onClick={props.close}>
          Cancel
        </button>
        <button
          type="submit"
          class="primary-button w-20 flex flex-row items-center justify-center h-full"
          disabled={mutation.isLoading || group.hasErrors()}
        >
          <Show
            when={!mutation.isLoading}
            fallback={<Spinner class="animate-spin" />}
          >
            Confirm
          </Show>
        </button>
      </div>
    </form>
  );
};
