import { Component, Show, createMemo } from "solid-js";
import { ProgramSet } from "../../api";
import { plural } from "../../util/setDisplay";

export const SetSummary: Component<{ sets: ProgramSet[] }> = (props) => {
  const nUnique = createMemo(() => {
    const s = new Set<string>();
    for (const set of props.sets) {
      s.add(set.movementId);
    }
    return s.size;
  });

  return (
    <Show when={props.sets.length}>
      <div class="mb-2">
        <p>
          {props.sets.length} Set{plural(props.sets.length)}, {nUnique()} unique movement
          {plural(nUnique())}
        </p>
      </div>
    </Show>
  );
};
