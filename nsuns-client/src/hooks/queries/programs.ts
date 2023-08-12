import { CreateMutationOptions, createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import {
  CreateProgram,
  Program,
  UpdateProgram,
  createProgram,
  deleteProgram,
  getProfilePrograms,
  updateProgram,
} from "../../api";
import { Accessor } from "solid-js";
import { QueryData, updateInArray } from "./util";
import { QueryKeys } from "./keys";
import { ProgramSummaryQueryData } from "./sets";

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
