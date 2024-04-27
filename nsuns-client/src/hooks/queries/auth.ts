import { createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import { QueryKeys } from "./keys";
import { login, loginAnonymous, logout, userInfo } from "../../api/auth";

export const useUserInfoQuery = () => {
  return createQuery({
    queryKey: QueryKeys.auth,
    queryFn: userInfo,
    cacheTime: Infinity,
    staleTime: Infinity,
  });
};

export const useLogoutMutation = () => {
  const queryClient = useQueryClient();
  const mutation = createMutation({
    mutationFn: logout,
    onSuccess: () => {
      queryClient.clear();
      window.location.replace("/login");
    },
  });

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

  return createMutation({
    mutationFn: login,
    onSuccess: invalidate,
  });
};

export const useLoginAnonymousMutation = () => {
  const invalidate = useInvalidateAfterLogin();
  return createMutation({
    mutationFn: loginAnonymous,
    onSuccess: invalidate,
  });
};
