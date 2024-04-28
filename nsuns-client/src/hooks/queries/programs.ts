import { CreateMutationOptions, createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import {
  CreateProgram,
  Program,
  ReorderSets,
  UpdateProgram,
  createProgram,
  deleteProgram,
  getProfilePrograms,
  getSetsPropByDay,
  reorderSets,
  updateProgram,
} from "../../api";
import { Accessor } from "solid-js";
import { QueryData, updateInArray } from "./util";
import { QueryKeys } from "./keys";
import { ProgramSummaryQueryData } from "./sets";
import { ProgramTemplate, createProgramFromTemplate } from "../../api/programTemplate";

export type ProgramsQueryData = QueryData<ReturnType<typeof useProgramsQuery>>;

export const useProgramsQuery = (profileId: Accessor<string>) => {
  const programsQuery = createQuery({
    queryKey: () => QueryKeys.programs.list(profileId()),
    queryFn: () => getProfilePrograms(profileId()),
    enabled: !!profileId,
  });
  return programsQuery;
};

export const useCreateProgram = <TError = unknown, TContext = unknown>(
  options?: Partial<CreateMutationOptions<Program, TError, CreateProgram, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createProgram,
    onSuccess: (program, ...args) => {
      options?.onSuccess?.(program, ...args);
      queryClient.setQueryData(QueryKeys.programs.list(program.owner), (programs?: ProgramsQueryData) =>
        programs ? [...programs, program] : undefined
      );
    },
  });
  return mutation;
};

export const useCreateProgramFromTemplate = <TError = unknown, TContext = unknown>(
  options?: Partial<CreateMutationOptions<Program, TError, ProgramTemplate, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createProgramFromTemplate,
    onSuccess: async (program, ...args) => {
      options?.onSuccess?.(program, ...args);
      queryClient.setQueryData(QueryKeys.programs.list(program.owner), (programs?: ProgramsQueryData) =>
        programs ? [...programs, program] : undefined
      );
      await queryClient.invalidateQueries({
        queryKey: QueryKeys.movements(),
      })
    },
  });
  return mutation;
};

export const useUpdateProgram = <TError = unknown, TContext = unknown>(
  options?: Partial<CreateMutationOptions<Program, TError, UpdateProgram, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: updateProgram,
    onSuccess: (program, ...args) => {
      options?.onSuccess?.(program, ...args);
      queryClient.setQueryData(QueryKeys.programs.list(program.owner), (programs?: ProgramsQueryData) =>
        updateInArray(programs, program, (p) => p.id === program.id)
      );
      queryClient.setQueryData(
        QueryKeys.programs.summary(program.id),
        (summary?: ProgramSummaryQueryData) =>
          summary && {
            ...summary,
            program,
          }
      );
    },
  });
  return mutation;
};

export const useDeleteProgram = <TError = unknown, TContext = unknown>(
  options?: Partial<CreateMutationOptions<Program, TError, string, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: deleteProgram,
    onSuccess: (program, id, ...args) => {
      options?.onSuccess?.(program, id, ...args);
      queryClient.setQueryData(
        QueryKeys.programs.list(program.owner),
        (programs?: ProgramsQueryData) => programs?.filter((p) => p.id !== id)
      );
    },
  });
  return mutation;
};

export const useReorderSets = <TError = unknown, TContext = unknown>(
  options?: Partial<CreateMutationOptions<string[], TError, ReorderSets, TContext>>
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: reorderSets,
    onMutate: (reorder, ...args) => {
      // optimistic update
      queryClient.setQueryData(QueryKeys.programs.summary(reorder.programId), (summary?: ProgramSummaryQueryData) => {
        const dayProp = getSetsPropByDay(reorder.day);

        if (summary) {
          const existingSets = summary[dayProp];

          const updatedItems = existingSets.slice();
          updatedItems.splice(reorder.to, 0, ...updatedItems.splice(reorder.from, 1));

          return {
            ...summary,
            [dayProp]: updatedItems,
          };
        }

        return undefined;
      });
      return options?.onMutate?.(reorder, ...args);
    },
    onError: async (error, reorder, ...args) => {
      // refetch if the reorder failed
      await queryClient.invalidateQueries({ queryKey: QueryKeys.programs.summary(reorder.programId) });
      options?.onError?.(error, reorder, ...args);
    },
    onSuccess: (setIds, reorder, ...args) => {
      options?.onSuccess?.(setIds, reorder, ...args);
      queryClient.setQueryData(QueryKeys.programs.summary(reorder.programId), (summary?: ProgramSummaryQueryData) => {
        const dayProp = getSetsPropByDay(reorder.day);
        const existingSets = summary?.[dayProp];
        return (
          summary && {
            ...summary,
            [dayProp]: setIds.map((id) => existingSets!.find((set) => set.id === id)),
          }
        );
      });
    },
  });
  return mutation;
};
