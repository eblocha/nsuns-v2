import { Component, For, Show } from "solid-js";
import { Day, Movement } from "../../api";
import { useCreateSet } from "../../hooks/queries/sets";
import { Spinner } from "../../icons/Spinner";
import { createControl, createControlGroup, required } from "../../hooks/forms";

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
    repsIsMinimum: createControl<boolean>(true),
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
      percentageOfMax: value.maxMovementId ? parseInt(value.maxMovementId) : null,
      programId: props.programId,
      reps: value.reps ? parseInt(value.reps) : null,
      repsIsMinimum: value.repsIsMinimum ?? false,
    });
  };

  return (
    <form
      class="grid grid-cols-3 border border-gray-700 p-4 rounded"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <h3 class="col-span-3 text-lg mb-2">Add Set</h3>
      <label>
        <span class="text-red-500">*</span>Movement:
        <select
          class="input ml-3"
          value={group.controls.movementId.value() ?? undefined}
          onChange={(e) => {
            group.controls.movementId.setValue(e.target.value);
            group.controls.movementId.setDirty(true);
          }}
          onBlur={() => group.controls.movementId.setTouched(true)}
        >
          <For each={props.movements}>
            {(movement) => <option value={movement.id}>{movement.name}</option>}
          </For>
        </select>
      </label>
      <label>
        Reps:
        <input
          class="input ml-3"
          type="number"
          min={0}
          value={group.controls.reps.value() ?? undefined}
        />
      </label>
      <label class="flex flex-row items-center">
        <input class="mr-3" type="checkbox" />
        Is Minimum?
      </label>

      <div class="col-span-3 flex flex-row items-center justify-end gap-2 mt-3">
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
