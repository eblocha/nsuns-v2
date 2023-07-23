import { Accessor, Component, JSX, createContext, useContext } from "solid-js";
import { Movement } from "../api";
import { Max } from "../api/maxes";
import { Reps } from "../api/reps";
import { MergedQueryState, combineQueries } from "../hooks/queries/util";
import { useMovementsQuery } from "../hooks/queries/movements";
import { useMaxesQuery } from "../hooks/queries/maxes";
import { useRepsQuery } from "../hooks/queries/reps";
import { useMovementMap } from "../hooks/useMovementMap";
import { useMovementsToMaxesMap } from "../hooks/useMovementsToMaxesMap";
import { useMovementsToRepsMap } from "../hooks/useMovementsToRepsMap";

type StatsContextData = {
  profileId: Accessor<string>;
  /**
   * Movement id to Movement.
   */
  movementMap: Accessor<Record<number, Movement>>;
  /**
   * Movement id to time-ordered (earliest first) maxes for that movement.
   */
  movementsToMaxesMap: Accessor<Record<number, Max[]>>;
  /**
   * Movement id to time-ordered (earliest first) reps for that movement.
   */
  movementsToRepsMap: Accessor<Record<number, Reps[]>>;
  queryState: MergedQueryState;
};

const EMPTY: never[] = [];

const StatsContext = createContext<StatsContextData>();

export const StatsProvider: Component<{
  profileId: string;
  children?: JSX.Element;
}> = (props) => {
  const movementsQuery = useMovementsQuery();
  const maxesQuery = useMaxesQuery(() => props.profileId);
  const repsQuery = useRepsQuery(() => props.profileId);

  const queryState = combineQueries(movementsQuery, maxesQuery);

  const movements = () => movementsQuery.data ?? EMPTY;
  const maxes = () => maxesQuery.data ?? EMPTY;
  const reps = () => repsQuery.data ?? EMPTY;

  const movementMap = useMovementMap(movements);
  const movementsToMaxesMap = useMovementsToMaxesMap(maxes);
  const movementsToRepsMap = useMovementsToRepsMap(reps);

  return (
    <StatsContext.Provider
      value={{
        profileId: () => props.profileId,
        movementMap,
        movementsToMaxesMap,
        movementsToRepsMap,
        queryState,
      }}
    >
      {props.children}
    </StatsContext.Provider>
  );
};

export const useStats = () => {
  const ctx = useContext(StatsContext);
  if (ctx === undefined) {
    throw new Error("StatsContext is undefined!");
  }
  return ctx;
};
