import { Accessor, Component, JSX, createContext, createMemo, useContext } from "solid-js";
import { ProgramSet, getSetsByDay } from "../../../api";
import { useProgramSummaryQuery } from "../../../hooks/queries/sets";
import { MergedQueryState } from "../../../hooks/queries/util";
import { Day, days } from "../../../util/days";
import { StatsProvider, useStats } from "../../../stats/StatsProvider";

type ProgramContextData = ReturnType<typeof useStats> & {
  programId: Accessor<string>;
  /**
   * Get an array of sets for a specific day
   */
  getSets: (day: Day) => ProgramSet[];
  /**
   * Unique movement ids that are referenced by this program.
   */
  relevantMovements: Accessor<string[]>;
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

  const getSets = (day: Day) => summaryQuery.data ? getSetsByDay(summaryQuery.data, day) : EMPTY;

  const relevantMovements = createMemo(() => {
    const uniqueIds: string[] = [];
    for (const day of days) {
      for (const set of getSets(day)) {
        if (!set.percentageOfMax) continue;
        if (!uniqueIds.includes(set.percentageOfMax)) {
          uniqueIds.push(set.percentageOfMax);
        }
      }
    }

    return uniqueIds;
  });

  return (
    <ProgramContext.Provider
      value={{
        ...stats,
        programId: () => props.programId,
        getSets,
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
