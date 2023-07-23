import { Accessor, Component, JSX, createContext, useContext } from "solid-js";
import { ProgramSet } from "../../../api";
import { useProgramSummaryQuery } from "../../../hooks/queries/sets";
import { MergedQueryState } from "../../../hooks/queries/util";
import { useSetMap } from "../../../hooks/useSetMap";
import { DayName } from "../../../util/days";
import { StatsProvider, useStats } from "../../../stats/StatsProvider";

type ProgramContextData = ReturnType<typeof useStats> & {
  programId: Accessor<number>;
  /**
   * Name of the day to the list of set definitions for the day.
   */
  setMap: Accessor<Record<DayName, ProgramSet[]>>;
  /**
   * Unique movement ids that are referenced by this program.
   */
  relevantMovements: Accessor<number[]>;
  queryState: MergedQueryState;
};

const EMPTY: never[] = [];

const ProgramContext = createContext<ProgramContextData>();

const InnerProvider: Component<{
  programId: string;
  children?: JSX.Element;
}> = (props) => {
  const stats = useStats();

  const summaryQuery = useProgramSummaryQuery(() => props.programId);

  const queryState: MergedQueryState = {
    error: () => stats.queryState.error() || summaryQuery.error,
    isError: () => stats.queryState.isError() || summaryQuery.isError,
    isLoading: () => stats.queryState.isLoading() || summaryQuery.isLoading,
    isSuccess: () => stats.queryState.isSuccess() && summaryQuery.isSuccess,
  };

  const sets = () => summaryQuery.data?.sets ?? EMPTY;

  const setMap = useSetMap(sets);

  const relevantMovements = () => {
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
        ...stats,
        programId: () => parseInt(props.programId),
        setMap,
        relevantMovements,
        queryState,
      }}
    >
      {props.children}
    </ProgramContext.Provider>
  );
};

export const ProgramProvider: Component<{
  programId: string;
  profileId: string;
  children?: JSX.Element;
}> = (props) => {
  return (
    <StatsProvider profileId={props.profileId}>
      <InnerProvider programId={props.programId}>
        {props.children}
      </InnerProvider>
    </StatsProvider>
  );
};

export const useProgram = () => {
  const ctx = useContext(ProgramContext);
  if (ctx === undefined) {
    throw new Error("ProgramContext is undefined!");
  }
  return ctx;
};
