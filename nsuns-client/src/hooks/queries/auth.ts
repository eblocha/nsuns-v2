import { createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import { QueryKeys } from "./keys";
import { logout, userInfo } from "../../api/auth";
import { useNavigate } from "@solidjs/router";
import { ApiError } from "../../api";

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

      const cache = queryClient.getQueryCache();

      // we manually set invalidated and error states so we don't attempt to fetch data again without auth
      cache.find(QueryKeys.auth())?.setState({
        error: new ApiError(401, "Unauthorized", ""),
        status: "error",
      });
      // important to clear cache _after_ setting state for auth, so the trial message disappears
      queryClient.clear();
    },
  });

  return mutation;
};
