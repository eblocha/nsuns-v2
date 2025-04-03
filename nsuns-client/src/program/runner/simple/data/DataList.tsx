import { Component, Match, Switch } from "solid-js";
import { useProgram } from "../../context/ProgramProvider";
import { createSmartAsyncDelay } from "../../../../hooks/asymmetricDelay";
import { currentSet, day } from "../../state";
import { getLatestMax } from "../../../../hooks/useMovementsToMaxesMap";
import { getLatestReps } from "../../../../hooks/useMovementsToRepsMap";
import { EditableCard } from "../../data/DataList";

export const DataList: Component = () => {
  const {
    getSets,
    movementMap,
    movementsToMaxesMap,
    movementsToRepsMap,
    profileId,
    queryState: { isLoading, isSuccess },
  } = useProgram();

  const delayedIsLoading = createSmartAsyncDelay(isLoading);

  const currentProgramSet = () => getSets(day())[currentSet()];
  const currentMovement = () => {
    const set = currentProgramSet();
    return set && movementMap()[set.movementId];
  };
  const currentMax = () => {
    const set = currentProgramSet();
    return set ? getLatestMax(movementsToMaxesMap(), set) : undefined;
  };
  const currentReps = () => {
    const set = currentProgramSet();
    return set ? getLatestReps(movementsToRepsMap(), set) : undefined;
  };

  return (
    <Switch>
      <Match when={delayedIsLoading()}>
        <div class="rounded shimmer h-32" />
        <div class="rounded shimmer h-32" />
      </Match>
      <Match when={isSuccess()}>
        <div class="flex flex-col gap-2">
          <div class="text-3xl">Maxes</div>
          <EditableCard
            profileId={profileId()}
            type="max"
            movement={currentMovement()}
            stat={currentMax()}
          />
        </div>
        <div class="flex flex-col gap-2">
          <div class="text-3xl">Reps</div>
          <EditableCard
            profileId={profileId()}
            type="reps"
            movement={currentMovement()}
            stat={currentReps()}
          />
        </div>
      </Match>
    </Switch>
  );
};
