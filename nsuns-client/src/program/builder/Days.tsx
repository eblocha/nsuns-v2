import { Component, For, Setter, Show, createSignal } from "solid-js";
import { ProgramSummary, getSetsByDay } from "../../api";
import { Plus } from "../../icons/Plus";
import { NewSet } from "./NewSet";
import { useMovementsQuery } from "../../hooks/queries/movements";
import { SetComponent } from "./Set";
import { SetSummary } from "./SetSummary";
import { ChevronDown } from "../../icons/ChevronDown";
import { Day, dayNames } from "../../util/days";

const EMPTY: never[] = [];

const TitleRow: Component<{
  index: number;
  day: string;
  hasSets: boolean;
  expanded: boolean;
  setExpanded: Setter<boolean[]>;
}> = (props) => {
  return (
    <div class="mb-2 flex flex-row items-center">
      <h3 class="text-lg">{props.day}</h3>
      <Show
        when={!props.hasSets}
        fallback={
          <button
            class="ml-auto text-sm text-button"
            classList={{
              "rotate-180": props.expanded,
            }}
            onClick={() =>
              props.setExpanded((expanded) => {
                const e = [...expanded];
                e[props.index] = !e[props.index];
                return e;
              })
            }
          >
            <ChevronDown />
          </button>
        }
      >
        <span class="italic opacity-80 text-sm ml-4">Rest Day</span>
      </Show>
    </div>
  );
};

export const Days: Component<{ summary: ProgramSummary }> = (props) => {
  const [addSetTo, setAddSetTo] = createSignal<number | null>(null);
  const [expanded, setExpanded] = createSignal(dayNames.map(() => true));
  const query = useMovementsQuery();

  const getSets = (index: number) => getSetsByDay(props.summary, index as Day);

  const movements = () => query.data ?? EMPTY;

  return (
    <ul>
      <For each={dayNames}>
        {(day, index) => {
          return (
            <li class="mb-4">
              <TitleRow
                day={day}
                hasSets={!!getSets(index())?.length}
                index={index()}
                setExpanded={setExpanded}
                expanded={expanded()[index()]!}
              />
              <ul>
                <Show
                  when={expanded()[index()]}
                  fallback={
                    <li>
                      <SetSummary sets={getSets(index())} />
                    </li>
                  }
                >
                  <For each={getSets(index())}>
                    {(set) => (
                      <li class="rounded border border-gray-700 mb-2">
                        <SetComponent
                          set={set}
                          movements={movements()}
                          dayIndex={index() as Day}
                          programId={props.summary.program.id}
                        />
                      </li>
                    )}
                  </For>
                </Show>

                <li>
                  <Show
                    when={addSetTo() === index()}
                    fallback={
                      <button
                        class="text-button-outline text-sm flex flex-row items-center justify-center gap-2"
                        disabled={addSetTo() !== null}
                        onClick={() => setAddSetTo(index())}
                      >
                        <Plus />
                        Add Set
                      </button>
                    }
                  >
                    <NewSet
                      close={() => setAddSetTo(null)}
                      dayIndex={index()}
                      programId={props.summary.program.id}
                      movements={movements()}
                    />
                  </Show>
                </li>
              </ul>
            </li>
          );
        }}
      </For>
    </ul>
  );
};
