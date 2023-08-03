import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  CreateProgramSet,
  ProgramSet,
  UpdateProgramSet,
  createSet,
  deleteSet,
  getProgramSummary,
  getSetsPropByDay,
  updateSet,
} from "../../api";
import { Accessor } from "solid-js";
import { QueryData, updateInArray } from "./util";
import { QueryKeys } from "./keys";
import { Day } from "../../util/days";

export type ProgramSummaryQueryData = QueryData<
  ReturnType<typeof useProgramSummaryQuery>
>;

export const useProgramSummaryQuery = (programId: Accessor<string>) => {
  return createQuery({
    queryKey: () => QueryKeys.programs.summary(programId()),
    queryFn: () => getProgramSummary(programId()),
    enabled: !!programId(),
  });
};

export const useCreateSet = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<ProgramSet, TError, CreateProgramSet, TContext>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createSet,
    onSuccess: (set, vars, ...args) => {
      options?.onSuccess?.(set, vars, ...args);
      queryClient.setQueryData(
        QueryKeys.programs.summary(vars.programId.toString()),
        (summary?: ProgramSummaryQueryData) => {
          const prop = getSetsPropByDay(vars.day);
          const current = summary?.[prop];

          if (!current) return summary;
          return { ...summary, [prop]: [...current, set] };
        }
      );
    },
  });

  return mutation;
};

export const useEditSet = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<
      ProgramSet,
      TError,
      UpdateProgramSet & { programId: string; day: Day },
      TContext
    >
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateSet,
    onSuccess: (set, variables, ...args) => {
      options?.onSuccess?.(set, variables, ...args);
      queryClient.setQueryData(
        QueryKeys.programs.summary(variables.programId),
        (summary?: ProgramSummaryQueryData) => {
          const prop = getSetsPropByDay(variables.day);
          return (
            summary && {
              ...summary,
              [prop]:
                updateInArray(summary[prop], set, (s) => s.id === set.id) ??
                summary[prop],
            }
          );
        }
      );
    },
  });

  return mutation;
};

export const useDeleteSet = <TError = unknown, TContext = unknown>(
  options?: Partial<
    CreateMutationOptions<
      void,
      TError,
      { id: string; meta: { programId: string; day: Day } },
      TContext
    >
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: ({ id }) => deleteSet(id),
    onSuccess: (v, vars, ...args) => {
      options?.onSuccess?.(v, vars, ...args);
      queryClient.setQueryData(
        QueryKeys.programs.summary(vars.meta.programId),
        (summary?: ProgramSummaryQueryData) => {
          const prop = getSetsPropByDay(vars.meta.day);
          return (
            summary && {
              ...summary,
              [prop]: summary[prop].filter((s) => s.id !== vars.id),
            }
          );
        }
      );
    },
  });
  return mutation;
};
