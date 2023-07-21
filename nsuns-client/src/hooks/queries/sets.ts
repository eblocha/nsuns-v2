import {
  CreateMutationOptions,
  createMutation,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  CreateProgramSet,
  ProgramSet,
  ProgramSummary,
  createSet,
} from "../../api";

export const useCreateSet = (
  options?: Partial<
    CreateMutationOptions<ProgramSet, unknown, CreateProgramSet, unknown>
  >
) => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    ...options,
    mutationFn: createSet,
    onSuccess: (set, ...args) => {
      queryClient.setQueryData(
        ["programs", set.programId],
        (summary?: ProgramSummary) => {
          if (!summary) return;

          return {
            program: summary.program,
            sets: [...summary.sets, set],
          };
        }
      );
      options?.onSuccess?.(set, ...args);
    },
  });

  return mutation;
};
