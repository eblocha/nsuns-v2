import { Component, For, Setter, Show, createMemo, createRenderEffect, createSignal } from "solid-js";
import { MovementTemplate, NewProgramTemplate, ProgramTemplate, TEMPLATES } from "../api/programTemplate";
import { useMovementsQuery } from "../hooks/queries/movements";
import { Select, SelectOption } from "../forms/Select";
import { Validator, createControl, required } from "../hooks/forms";
import { Movement } from "../api";
import { useCreateProgramFromTemplate } from "../hooks/queries/programs";
import { createSmartAsyncDelay } from "../hooks/asymmetricDelay";
import { Spinner } from "../icons/Spinner";
import { Warning } from "../icons/Warning";
import { displayError } from "../util/errors";
import { useNavigateToProgram } from "../hooks/navigation";
import { useParams } from "@solidjs/router";
import { Input } from "../forms/Input";

type TranslatedMovement = {
  id?: string;
  name: string;
  description: string | null;
};

const TemplatedMovementRow: Component<{
  movement: TranslatedMovement;
  names: string[];
  setName: (name: string) => void;
  existingId: string;
  setExistingId: (id: string) => void;
  options: Movement[];
  index: number;
}> = (props) => {
  const movementOptions = createMemo<SelectOption[]>(() =>
    props.options.map((movement) => ({
      value: movement.id.toString(),
      name: movement.name,
    }))
  );

  const isUnique: Validator<string> = (name: string) => {
    return {
      isNotUnique:
        props.names.some((n, idx) => idx !== props.index && name === n) ||
        props.options.some((movement) => movement.name === name),
    };
  };

  const nameControl = createControl<string>(props.names[props.index] ?? "", {
    validators: [required(), isUnique],
    alwaysShowError: true,
  });
  const selectControl = createControl(props.existingId);

  createRenderEffect(() => {
    nameControl.setValue(props.names[props.index] ?? "");
  });

  createRenderEffect(() => {
    selectControl.setValue(props.existingId);
  });

  return (
    <>
      <Select
        class="input h-8"
        control={{
          ...selectControl,
          setValue: (val) => {
            if (typeof val === "function") {
              props.setExistingId(val(selectControl.value()));
            } else {
              props.setExistingId(val);
            }
            return selectControl.setValue(val);
          },
        }}
        options={movementOptions()}
        emptyOption="<create new>"
      />
      <Show
        when={selectControl.value() === ""}
        fallback={<span class="self-center h-8 flex items-center">{props.movement.name}</span>}
      >
        <Input
          class="input h-8"
          control={{
            ...nameControl,
            setValue: (val) => {
              if (typeof val === "function") {
                props.setName(val(nameControl.value()));
              } else {
                props.setName(val);
              }
              return nameControl.setValue(val);
            },
          }}
          required={selectControl.value() === ""}
        />
      </Show>
    </>
  );
};

export const TemplatedProgram: Component<{ template: NewProgramTemplate; close: () => void; cancel: () => void }> = (
  props
) => {
  const params = useParams<{ profileId: string }>();
  const navigateToProgram = useNavigateToProgram();

  const movementQuery = useMovementsQuery();
  const mutation = useCreateProgramFromTemplate({
    onSuccess: (program) => {
      navigateToProgram(program.id);
      props.close();
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

  const [names, setNames] = createSignal(translatedMovements().map(() => ""));
  const [selectedIds, setSelectedIds] = createSignal(translatedMovements().map((movement) => movement.id ?? ""));

  const onSubmit = (event: SubmitEvent) => {
    event.preventDefault();

    if (mutation.isLoading) {
      return;
    }

    const translated = translatedMovements();
    const movementTemplates: MovementTemplate[] = [];

    for (let i = 0; i < translated.length; i++) {
      // SAFETY: these arrays are all created from the value of `translatedMovements`
      const id = selectedIds()[i]!;
      const name = names()[i]!;
      const movement = translated[i]!;

      if (id) {
        movementTemplates.push({
          type: "existing",
          id,
        });
      } else {
        movementTemplates.push({
          type: "new",
          name,
          description: movement.description,
        });
      }
    }

    const template: ProgramTemplate = {
      ...props.template,
      owner: params.profileId,
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
      <div class="grid grid-cols-3 gap-2">
        <div class="font-bold">Template</div>
        <div class="font-bold">Mapped To</div>
        <div class="font-bold">Name</div>
        <For each={translatedMovements()}>
          {(movement, index) => (
            <>
              <span class="self-center h-8 flex items-center">{movement.name}</span>
              <TemplatedMovementRow
                names={names()}
                existingId={selectedIds()[index()]!}
                setName={(value) => {
                  const idx = index();
                  setNames((names) => {
                    return names.map((name, index) => (index === idx ? value : name));
                  });
                }}
                setExistingId={(value) => {
                  const idx = index();
                  setSelectedIds((ids) => {
                    return ids.map((id, index) => (index === idx ? value : id));
                  });
                }}
                movement={movement}
                options={movementList()}
                index={index()}
              />
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
