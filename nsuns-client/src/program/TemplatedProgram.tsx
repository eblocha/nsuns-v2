import { Component, For, Setter, Show, createEffect, createMemo, createRenderEffect, createSignal } from "solid-js";
import {
  MovementTemplate,
  NewProgramTemplate,
  ProgramTemplate,
  TEMPLATES,
} from "../api/programTemplate";
import { useMovementsQuery } from "../hooks/queries/movements";
import { Select, SelectOption } from "../forms/Select";
import { Control, createControl } from "../hooks/forms";
import { Movement } from "../api";
import { useCreateProgramFromTemplate } from "../hooks/queries/programs";
import { createSmartAsyncDelay } from "../hooks/asymmetricDelay";
import { Spinner } from "../icons/Spinner";
import { Warning } from "../icons/Warning";
import { displayError } from "../util/errors";

type TranslatedMovement = {
  id?: string;
  name: string;
  description: string | null;
};

const TemplatedMovementRow: Component<{
  movement: TranslatedMovement;
  control: Control<string>;
  options: Movement[];
}> = (props) => {
  const movementOptions = createMemo<SelectOption[]>(() =>
    props.options.map((movement) => ({
      value: movement.id.toString(),
      name: movement.name,
    }))
  );

  createEffect(() => props.control.reset(props.movement.id ?? ""));

  return (
    <Select
      class="input w-full"
      control={props.control}
      options={movementOptions()}
      emptyOption="<create new>"
    />
  );
};

export const TemplatedProgram: Component<{ template: NewProgramTemplate; close: () => void; cancel: () => void }> = (
  props
) => {
  const movementQuery = useMovementsQuery();
  const mutation = useCreateProgramFromTemplate({
    onSuccess: () => {
      props.cancel();
    },
  });

  const isCreating = createSmartAsyncDelay(() => mutation.isLoading);

  const movementList = () => movementQuery.data ?? [];

  const translatedMovements = createMemo(() => {
    return props.template.movements
      .map((movementTemplate) => {
        const existing = movementList().find((movement) => movementTemplate.name === movement.name);

        if (existing) {
          return {
            name: existing.name,
            description: existing.description,
            id: existing.id,
          };
        }

        return {
          name: movementTemplate.name,
          description: movementTemplate.description,
        };
      })
      .filter((name): name is TranslatedMovement => !!name);
  });

  const controls = createMemo(() =>
    translatedMovements().map((movement) => ({ control: createControl(movement.id ?? ""), movement }))
  );

  const onSubmit = (event: SubmitEvent) => {
    event.preventDefault();

    if (mutation.isLoading) {
      return;
    }

    const movementTemplates: MovementTemplate[] = controls().map((value) => {
      const id = value.control.value();

      if (id) {
        return {
          type: "existing",
          id,
        } satisfies MovementTemplate;
      } else {
        return {
          type: "new",
          name: value.movement.name,
          description: value.movement.description,
        } satisfies MovementTemplate;
      }
    });

    const template: ProgramTemplate = {
      ...props.template,
      movements: movementTemplates,
    };

    mutation.mutate(template);
  };

  return (
    <form
      onSubmit={onSubmit}
      class="flex flex-col gap-4"
    >
      <h2 class="text-lg">Template: {props.template.name}</h2>
      <hr class="border-gray-600" />
      <div class="grid grid-cols-2 gap-2">
        <div class="font-bold">Template Movement</div>
        <div class="font-bold">Mapped To</div>
        <For each={controls()}>
          {(control) => (
            <>
              <span>{control.movement.name}</span>
              <div>
                <TemplatedMovementRow
                  control={control.control}
                  movement={control.movement}
                  options={movementList()}
                />
              </div>
            </>
          )}
        </For>
      </div>
      <div class="flex gap-2 justify-end">
        <button
          class="text-button"
          onClick={props.cancel}
          type="button"
        >
          Choose Another
        </button>
        <button
          class="secondary-button"
          onClick={props.close}
          type="button"
        >
          Cancel
        </button>
        <button
          class="primary-button w-20 flex justify-center items-center"
          type="submit"
          disabled={isCreating()}
        >
          <Show
            when={!isCreating()}
            fallback={<Spinner class="animate-spin" />}
          >
            Create
          </Show>
        </button>
      </div>
      <Show when={!isCreating() && mutation.isError}>
        <div class="col-span-2 flex flex-row items-center justify-end gap-4 mt-2">
          <span>
            <Warning class="text-red-500" />
          </span>
          {displayError(mutation.error, "create program")}
        </div>
      </Show>
    </form>
  );
};

export const TemplatedPrograms: Component<{ close: () => void; setIsEditing: Setter<boolean> }> = (props) => {
  const [templateSelected, setTemplateSelected] = createSignal<NewProgramTemplate | null>(null);

  createRenderEffect(() => {
    props.setIsEditing(() => templateSelected() !== null);
  });

  return (
    <Show
      when={templateSelected() === null}
      fallback={
        <TemplatedProgram
          template={templateSelected()!}
          cancel={() => setTemplateSelected(null)}
          close={props.close}
        />
      }
    >
      <div class="flex flex-row gap-2 items-stretch flex-wrap">
        <For each={TEMPLATES}>
          {(template) => (
            <button
              class="rounded-md border border-gray-600  bg-gray-800 hover:bg-gray-600 w-28 h-28 flex items-center justify-center p-8"
              onClick={() => setTemplateSelected(template)}
            >
              {template.name}
            </button>
          )}
        </For>
      </div>
    </Show>
  );
};
