import { useMutation, useQuery, useQueryClient } from "@tanstack/solid-query";
import { QueryKeys } from "./keys";
import { login, loginAnonymous, logout, userInfo } from "../../api/auth";

export const useUserInfoQuery = () => {
  return useQuery(() => ({
    queryKey: QueryKeys.auth(),
    queryFn: userInfo,
    cacheTime: Infinity,
    staleTime: Infinity,
  }));
};

export const useLogoutMutation = () => {
  const queryClient = useQueryClient();
  const mutation = useMutation(() => ({
    mutationFn: logout,
    onSuccess: () => {
      queryClient.clear();
      window.location.replace("/login");
    },
  }));

  return mutation;
};

const useInvalidateAfterLogin = () => {
  const queryClient = useQueryClient();

  return () => {
    queryClient.clear();
    window.location.replace("/");
  };
};

export const useLoginMutation = () => {
  const invalidate = useInvalidateAfterLogin();

  return useMutation(() => ({
    mutationFn: login,
    onSuccess: invalidate,
  }));
};

export const useLoginAnonymousMutation = () => {
  const invalidate = useInvalidateAfterLogin();
  return useMutation(() => ({
    mutationFn: loginAnonymous,
    onSuccess: invalidate,
  }));
};
