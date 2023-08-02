import { Component } from "solid-js";
import { Forward } from "../../icons/Forward";
import { currentSet, day, setCurrentSet } from "./state";
import { useProgram } from "./context/ProgramProvider";

const NextSet: Component<{ nSets: number }> = (props) => {
  return (
    <button
      class="text-button"
      disabled={currentSet() >= props.nSets - 1}
      onClick={() => {
        setCurrentSet((curr) => (curr >= props.nSets - 1 ? curr : curr + 1));
      }}
      title="Next Set"
    >
      <Forward />
    </button>
  );
};

const PrevSet: Component = () => {
  return (
    <button
      class="text-button"
      disabled={currentSet() === 0}
      onClick={() => {
        setCurrentSet((curr) => (curr === 0 ? curr : curr - 1));
      }}
      title="Previous Set"
    >
      <Forward class="rotate-180" />
    </button>
  );
};

export const Tools: Component = () => {
  const { getSets } = useProgram();

  return (
    <div class="flex flex-row items-center">
      <PrevSet />
      <NextSet nSets={getSets(day()).length} />
    </div>
  );
};
