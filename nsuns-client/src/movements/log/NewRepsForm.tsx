import { Component, Show } from "solid-js";
import { useCreateRepsMutation } from "../../hooks/queries/reps";
import { createControl, required } from "../../hooks/forms";
import { Input } from "../../forms/Input";
import { Warning } from "../../icons/Warning";
import { displayError } from "../../util/errors";

export const NewRepsForm: Component<{
  profileId: string;
  movementId: string;
  onClose: () => void;
}> = (props) => {
  const mutation = useCreateRepsMutation({
    onSuccess: props.onClose,
  });

  const amount = createControl<string>("", { validators: [required()] });

  const disableSubmit = () => mutation.isLoading || amount.hasErrors();

  const onSubmit = () => {
    if (disableSubmit()) return;

    mutation.mutate({
      profileId: props.profileId,
      amount: parseInt(amount.value()),
      movementId: props.movementId,
    });
  };

  return (
    <form
      class="flex flex-row items-stretch gap-4 w-full flex-wrap"
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
    >
      <div class="flex-grow flex flex-row items-center gap-2">
        <div class="flex-grow">
          <Input
            control={amount}
            name="max-amount"
            type="number"
            class="input w-full"
            step={1}
            autofocus={true}
          />
        </div>
        <span>reps</span>
      </div>
      <div class="flex flex-row items-center justify-end gap-2">
        <button class="secondary-button" type="button" onClick={props.onClose}>
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
        <div class="w-full flex flex-row items-center justify-end gap-4">
          <span>
            <Warning class="text-red-500" />
          </span>
          {displayError(mutation.error, "log reps")}
        </div>
      </Show>
    </form>
  );
};
