import {
  CreateMutationOptions,
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/solid-query";
import {
  CreateProfile,
  Profile,
  createProfile,
  getProfiles,
  updateProfile,
} from "../../api";
import { QueryKeys } from "./keys";
import { QueryData, updateInArray } from "./util";

export type ProfilesQueryData = QueryData<
  ReturnType<typeof createProfileQuery>
>;

export const createProfileQuery = () =>
  createQuery(QueryKeys.profiles, getProfiles);

export const createCreateProfileMutation = <
  TError = unknown,
  TContext = unknown
>(
  options?: Partial<
    CreateMutationOptions<Profile, TError, CreateProfile, TContext>
  >
) => {
  const queryClient = useQueryClient();

  return createMutation({
    mutationFn: createProfile,
    ...options,
    onSuccess: (profile, ...args) => {
      options?.onSuccess?.(profile, ...args);
      queryClient.setQueryData(
        QueryKeys.profiles(),
        (profiles?: ProfilesQueryData) => profiles && [...profiles, profile]
      );
    },
  });
};

export const createUpdateProfileMutation = <
  TError = unknown,
  TContext = unknown
>(
  options?: Partial<CreateMutationOptions<Profile, TError, Profile, TContext>>
) => {
  const queryClient = useQueryClient();

  return createMutation({
    mutationFn: updateProfile,
    ...options,
    onSuccess: (profile, ...args) => {
      options?.onSuccess?.(profile, ...args);
      queryClient.setQueryData(
        QueryKeys.profiles(),
        (profiles?: ProfilesQueryData) =>
          profiles &&
          updateInArray(profiles, profile, (p) => p.id === profile.id)
      );
    },
  });
};
