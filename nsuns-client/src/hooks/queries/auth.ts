import { createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import { QueryKeys } from "./keys";
import { logout, userInfo } from "../../api/auth";
import { useNavigate } from "@solidjs/router";

export const useUserInfoQuery = () => {
  return createQuery({
    queryKey: QueryKeys.auth,
    queryFn: userInfo,
  });
};

export const useLogoutMutation = () => {
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const mutation = createMutation({
    mutationFn: logout,
    onSuccess: () => {
      navigate("/login");
      queryClient.clear();
    },
  });

  return mutation;
};
