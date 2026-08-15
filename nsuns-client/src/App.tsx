import { Route, Router, useLocation } from "@solidjs/router";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { Component, ParentComponent, Show, createEffect } from "solid-js";

import { ApiError } from "./api";
import { DELAY_BEFORE_ASYNC_MS, SPINNER_DELAY_MS, createSmartAsyncDelay } from "./hooks/asymmetricDelay";
import { useNavigateToLogin, useNavigateToProfileHome } from "./hooks/navigation";
import { useUserInfoQuery } from "./hooks/queries/auth";
import { Spinner } from "./icons/Spinner";
import { ExpiryWarning } from "./login/ExpiryWarning";
import { Login } from "./login/Login";
import { NotFound } from "./NotFound";
import { CreateProfile } from "./profile/entry/CreateProfile";
import { SelectProfile } from "./profile/entry/SelectProfile";
import { ProfileHome } from "./profile/ProfileHome";
import { ProgramBuilder } from "./program/builder/ProgramBuilder";
import { SimpleProgramRunner } from "./program/runner/simple/SimpleProgramRunner";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      staleTime: 200,
      retry: (count, error) => {
        if (error instanceof ApiError) {
          if (error.status < 500 && error.status >= 400) {
            return false;
          }

          return count > 3;
        }
        return count < 3;
      },
    },
  },
});

const AppLayout: ParentComponent = (props) => {
  const userInfo = useUserInfoQuery();
  const navigateToLogin = useNavigateToLogin();
  const navigateToProfileHome = useNavigateToProfileHome();
  const location = useLocation();

  createEffect(() => {
    if (userInfo.isSuccess && userInfo.data === null && location.pathname !== "/login") {
      navigateToLogin();
    } else if (userInfo.isSuccess && userInfo.data !== null && location.pathname === "/login") {
      navigateToProfileHome();
    }
  });

  const isUserFetching = createSmartAsyncDelay(() => userInfo.isLoading, DELAY_BEFORE_ASYNC_MS, SPINNER_DELAY_MS);

  return (
    <Show
      when={!isUserFetching()}
      fallback={
        <div class="w-full h-full flex items-center justify-center gap-4">
          <Spinner class="animate-spin text-2xl" />
        </div>
      }
    >
      {props.children}
      <ExpiryWarning />
    </Show>
  );
};

const RoutingApp: Component = () => {
  return (
    <Router root={AppLayout}>
      <Route
        path="/"
        component={SelectProfile}
      />
      <Route
        path="/login"
        component={Login}
      />
      <Route
        path="/profile/new"
        component={CreateProfile}
      />
      <Route
        path="/profile/:profileId"
        component={ProfileHome}
      >
        <Route path="/" />
        <Route
          path="program/:programId"
          component={ProgramBuilder}
        />
      </Route>
      <Route
        path="/profile/:profileId/program/:programId/run"
        component={SimpleProgramRunner}
      />
      <Route
        path="*"
        component={NotFound}
      />
    </Router>
  );
};

export const App: Component = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <RoutingApp />
    </QueryClientProvider>
  );
};
