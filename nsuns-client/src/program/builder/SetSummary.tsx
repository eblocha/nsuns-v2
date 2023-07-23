import { Component, Show, createMemo } from "solid-js";
import { ProgramSet } from "../../api";

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
          {props.sets.length} Set{props.sets.length === 1 ? "" : "s"},{" "}
          {nUnique()} unique movement{nUnique() === 1 ? "" : "s"}
        </p>
      </div>
    </Show>
  );
};
