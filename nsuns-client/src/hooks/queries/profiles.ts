import { MutationOptions, useMutation, useQuery, useQueryClient } from "@tanstack/solid-query";

import { CreateProfile, Profile, createProfile, deleteProfile, getProfiles, updateProfile } from "../../api";
import { QueryKeys } from "./keys";
import { QueryData, updateInArray } from "./util";

export type ProfilesQueryData = QueryData<ReturnType<typeof createProfileQuery>>;

export const createProfileQuery = () => useQuery(() => ({ queryKey: QueryKeys.profiles(), queryFn: getProfiles }));

export const createCreateProfileMutation = <TError = unknown, TContext = unknown>(
  options?: Partial<MutationOptions<Profile, TError, CreateProfile, TContext>>
) => {
  const queryClient = useQueryClient();

  return useMutation(() => ({
    mutationFn: createProfile,
    ...options,
    onSuccess: (profile, ...args) => {
      options?.onSuccess?.(profile, ...args);
      queryClient.setQueryData(
        QueryKeys.profiles(),
        (profiles?: ProfilesQueryData) => profiles && [...profiles, profile]
      );
    },
  }));
};

export const createUpdateProfileMutation = <TError = unknown, TContext = unknown>(
  options?: Partial<MutationOptions<Profile, TError, Profile, TContext>>
) => {
  const queryClient = useQueryClient();

  return useMutation(() => ({
    mutationFn: updateProfile,
    ...options,
    onSuccess: (profile, ...args) => {
      options?.onSuccess?.(profile, ...args);
      queryClient.setQueryData(
        QueryKeys.profiles(),
        (profiles?: ProfilesQueryData) => profiles && updateInArray(profiles, profile, (p) => p.id === profile.id)
      );
    },
  }));
};

export const createDeleteProfileMutation = <TError = unknown, TContext = unknown>(
  options?: Partial<MutationOptions<Profile, TError, string, TContext>>
) => {
  const queryClient = useQueryClient();

  return useMutation(() => ({
    mutationFn: deleteProfile,
    ...options,
    onSuccess: (profile, ...args) => {
      options?.onSuccess?.(profile, ...args);
      queryClient.setQueryData(
        QueryKeys.profiles(),
        (profiles?: ProfilesQueryData) => profiles && profiles.filter((p) => p.id !== profile.id)
      );
    },
  }));
};
