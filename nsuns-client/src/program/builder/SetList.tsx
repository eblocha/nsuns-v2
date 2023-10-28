import { Component, For, createSignal } from "solid-js";
import { DragEventHandler } from "@thisbeyond/solid-dnd";
import { DragDropProvider, DragDropSensors, DragOverlay, SortableProvider, closestCenter } from "@thisbeyond/solid-dnd";
import { Movement, ProgramSet } from "../../api";
import { SetComponent, displaySet } from "./Set";
import { Day } from "../../util/days";
import { useReorderSets } from "../../hooks/queries/programs";

export const SetList: Component<{
  sets: ProgramSet[];
  movements: Movement[];
  dayIndex: Day;
  programId: string;
}> = (props) => {
  const { mutate } = useReorderSets();

  const reorderSets = (from: number, to: number) => {
    mutate({
      programId: props.programId,
      day: props.dayIndex,
      from,
      to,
    });
  };

  const [activeItem, setActiveItem] = createSignal<number | null>(null);

  const setIds = () => props.sets.map((set) => set.id);

  const activeItemDescription = () => {
    const index = activeItem();
    const set = index !== null && props.sets[index];
    if (set) {
      return displaySet(set, props.movements);
    }

    return null;
  };

  const onDragStart: DragEventHandler = ({ draggable }) =>
    setActiveItem(setIds().indexOf(draggable.id as string) ?? null);

  const onDragEnd: DragEventHandler = ({ draggable, droppable }) => {
    if (draggable && droppable) {
      const ids = setIds();
      const fromIndex = ids.indexOf(draggable.id as string);
      const toIndex = ids.indexOf(droppable.id as string);
      if (fromIndex !== toIndex) {
        reorderSets(fromIndex, toIndex);
      }
    }
  };

  return (
    <DragDropProvider
      onDragStart={onDragStart}
      onDragEnd={onDragEnd}
      collisionDetector={closestCenter}
    >
      <DragDropSensors />
      <SortableProvider ids={setIds()}>
        <For each={props.sets}>
          {(set) => (
            <li class="rounded border border-gray-700 mb-2">
              <SetComponent
                set={set}
                movements={props.movements}
                dayIndex={props.dayIndex}
                programId={props.programId}
              />
            </li>
          )}
        </For>
      </SortableProvider>
      <DragOverlay>
        <div class="p-2 rounded border border-gray-700 mb-2 bg-gray-800">{activeItemDescription()}</div>
      </DragOverlay>
    </DragDropProvider>
  );
};
