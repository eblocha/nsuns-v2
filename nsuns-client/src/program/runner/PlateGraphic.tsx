import { Component, createMemo, For, Match, Show, Switch } from "solid-js";
import { calculatePlates, PlateCount } from "../../util/plates";
import style from "./PlateGraphic.module.css";

const Plate45: Component = () => {
  return <div class="flex items-center justify-center w-10 h-full bg-blue-700 rounded">45</div>;
};

const Plate25: Component = () => {
  return <div class="flex items-center justify-center w-9 h-full bg-green-700 rounded">25</div>;
};

const Plate10: Component = () => {
  return <div class="flex items-center justify-center w-7 h-full bg-gray-800 rounded">10</div>;
};

const Plate5: Component = () => {
  return <div class="flex items-center justify-center w-7 h-1/2 bg-gray-900 rounded">5</div>;
};

const Plate2p5: Component = () => {
  return <div class="flex items-center justify-center w-7 h-1/2 bg-slate-700 rounded">2.5</div>;
};

const PlateGroup: Component<{ plate: PlateCount }> = (props) => {
  const arr = createMemo(() => Array.from({ length: props.plate.count }));

  return (
    <Switch>
      <Match when={props.plate.weight === 45}>
        <For each={arr()}>{() => <Plate45 />}</For>
      </Match>
      <Match when={props.plate.weight === 25}>
        <For each={arr()}>{() => <Plate25 />}</For>
      </Match>
      <Match when={props.plate.weight === 10}>
        <For each={arr()}>{() => <Plate10 />}</For>
      </Match>
      <Match when={props.plate.weight === 5}>
        <For each={arr()}>{() => <Plate5 />}</For>
      </Match>
      <Match when={props.plate.weight === 2.5}>
        <For each={arr()}>{() => <Plate2p5 />}</For>
      </Match>
    </Switch>
  );
};

export const PlateGraphic: Component<{ weight: number | undefined }> = (props) => {
  const plates = createMemo(() => {
    if (props.weight) {
      return calculatePlates(props.weight, 45, [45, 25, 10, 5, 2.5]);
    }

    return [];
  });

  const reversePlates = createMemo(() => {
    return [...plates()].reverse();
  });

  return (
    <Show when={plates().length}>
      <div
        class="relative flex flex-row justify-between items-stretch h-32 w-full max-w-2xl"
        classList={{ [style.barbell!]: true }}
      >
        <div class="pl-6 flex items-center">
          <For each={reversePlates()}>{(plate) => <PlateGroup plate={plate} />}</For>
        </div>
        <div class="pr-6 flex items-center">
        <For each={plates()}>{(plate) => <PlateGroup plate={plate} />}</For>
        </div>
      </div>
    </Show>
  );
};
