import { Component, Show } from "solid-js";
import { useMovementsQuery } from "../../../hooks/queries/movements";
import { useCreateMaxMutation } from "../../../hooks/queries/maxes";
import {
  createControl,
  createControlGroup,
  required,
} from "../../../hooks/forms";
import { Input } from "../../../forms/Input";
import { Select } from "../../../forms/Select";
import { Warning } from "../../../icons/Warning";
import { displayError } from "../../../util/errors";

export const NewMaxForm: Component<{ profileId: string; close: () => void }> = (
  props
) => {
  const movementsQuery = useMovementsQuery();
  const mutation = useCreateMaxMutation({
    onSuccess: props.close,
  });

  const group = createControlGroup({
    amount: createControl<string>("", { validators: [required()] }),
    movementId: createControl<string>("", { validators: [required()] }),
  });

  const disableSubmit = () => mutation.isLoading || group.hasErrors();

  const onSubmit = () => {
    if (disableSubmit()) return;

    const value = group.value();

    mutation.mutate({
      profileId: props.profileId,
      amount: parseFloat(value.amount),
      movementId: value.movementId,
    });
  };

  return (
    <form
      class="grid grid-cols-7 gap-4"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <Select
        control={group.controls.movementId}
        name="max-movement"
        class="input col-span-3"
        options={movementsQuery.data?.map((mv) => ({
          value: mv.id.toString(),
          name: mv.name,
        }))}
      />
      <div class="flex flex-row items-center gap-2 col-span-2">
        <div class="flex-grow">
          <Input
            control={group.controls.amount}
            name="max-amount"
            type="number"
            class="input w-full"
            step={5}
          />
        </div>
        <span>lbs</span>
      </div>
      <div class="flex flex-row items-center justify-end gap-2 col-span-2">
        <button class="secondary-button" type="button" onClick={props.close}>
          Cancel
        </button>
        <button
          class="primary-button border border-transparent"
          disabled={disableSubmit()}
        >
          Log
        </button>
      </div>
      <Show when={mutation.isError}>
        <div class="col-span-7 flex flex-row items-center justify-end gap-4">
          <span>
            <Warning class="text-red-500" />
          </span>
          {displayError(mutation.error, "log max")}
        </div>
      </Show>
    </form>
  );
};
