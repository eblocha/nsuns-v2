import { Accessor, Component, JSX, createContext, useContext } from "solid-js";
import { Movement, ProgramSet } from "../../../api";
import { Max } from "../../../api/maxes";
import { useProgramSummaryQuery } from "../../../hooks/queries/sets";
import { useMovementsQuery } from "../../../hooks/queries/movements";
import { useMaxesQuery } from "../../../hooks/queries/maxes";
import { MergedQueryState, combineQueries } from "../../../hooks/queries/util";
import { useSetMap } from "../../../hooks/useSetMap";
import { useMovementMap } from "../../../hooks/useMovementMap";
import { useMovementsToMaxesMap } from "../../../hooks/useMovementsToMaxesMap";

type ProgramContextData = {
  programId: Accessor<number>;
  profileId: Accessor<string>;
  /**
   * Name of the day to the list of set definitions for the day.
   */
  setMap: Accessor<Record<string, ProgramSet[]>>;
  /**
   * Movement id to Movement.
   */
  movementMap: Accessor<Record<number, Movement>>;
  /**
   * Movement id to time-ordered (earliest first) maxes for that movement.
   */
  movementsToMaxesMap: Accessor<Record<number, Max[]>>;
  /**
   * Unique movement ids that have a max-percentage reference or a direct reference in this program.
   */
  relevantMovements: Accessor<number[]>;
  queryState: MergedQueryState;
};

const EMPTY: never[] = [];

const ProgramContext = createContext<ProgramContextData>();

export const ProgramProvider: Component<{
  programId: string;
  profileId: string;
  children?: JSX.Element;
}> = (props) => {
  const summaryQuery = useProgramSummaryQuery(() => props.programId);
  const movementsQuery = useMovementsQuery();
  const maxesQuery = useMaxesQuery(() => props.profileId);

  const queryState = combineQueries(summaryQuery, movementsQuery, maxesQuery);

  const sets = () => summaryQuery.data?.sets ?? EMPTY;
  const movements = () => movementsQuery.data ?? EMPTY;
  const maxes = () => maxesQuery.data ?? EMPTY;

  const setMap = useSetMap(sets);
  const movementMap = useMovementMap(movements);
  const movementsToMaxesMap = useMovementsToMaxesMap(maxes);

  const movementsWithMaxInProgram = () => {
    const uniqueIds: number[] = [];
    for (const set of sets()) {
      if (!uniqueIds.includes(set.movementId)) {
        uniqueIds.push(set.movementId);
      }

      if (!set.percentageOfMax) continue;
      if (!uniqueIds.includes(set.percentageOfMax)) {
        uniqueIds.push(set.percentageOfMax);
      }
    }

    return uniqueIds;
  };

  return (
    <ProgramContext.Provider
      value={{
        profileId: () => props.profileId,
        programId: () => parseInt(props.programId),
        setMap,
        movementMap,
        movementsToMaxesMap,
        relevantMovements: movementsWithMaxInProgram,
        queryState,
      }}
    >
      {props.children}
    </ProgramContext.Provider>
  );
};

export const useProgram = () => {
  const ctx = useContext(ProgramContext);
  if (ctx === undefined) {
    throw new Error("ProgramContext is undefined!");
  }
  return ctx;
};
