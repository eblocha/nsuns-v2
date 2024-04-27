import { createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import { QueryKeys } from "./keys";
import { login, loginAnonymous, logout, userInfo } from "../../api/auth";
import { useNavigateToLogin, useNavigateToProfileHome } from "../navigation";

export const useUserInfoQuery = () => {
  return createQuery({
    queryKey: QueryKeys.auth,
    queryFn: userInfo,
  });
};

export const useLogoutMutation = () => {
  const navigate = useNavigateToLogin();
  const queryClient = useQueryClient();
  const mutation = createMutation({
    mutationFn: logout,
    onSuccess: () => {
      navigate();
      queryClient.setQueryData(QueryKeys.auth(), null);
      // important to clear cache _after_ setting state for auth, so the trial message disappears
      queryClient.clear();
    },
  });

  return mutation;
};

const useInvalidateAfterLogin = () => {
  const navigate = useNavigateToProfileHome();
  const queryClient = useQueryClient();

  return async () => {
    await queryClient.invalidateQueries({
      exact: true,
      queryKey: QueryKeys.auth(),
    });
    // important to navigate after invalidating so we don't get auto-routed back to login
    navigate();
  };
};

export const useLoginMutation = () => {
  return createMutation({
    mutationFn: login,
    onSuccess: useInvalidateAfterLogin(),
  });
};

export const useLoginAnonymousMutation = () => {
  return createMutation({
    mutationFn: loginAnonymous,
    onSuccess: useInvalidateAfterLogin(),
  });
};
