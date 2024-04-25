import { Route, Router, Routes } from "@solidjs/router";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { SelectProfile } from "./profile/entry/SelectProfile";
import { CreateProfile } from "./profile/entry/CreateProfile";
import { ProfileHome } from "./profile/ProfileHome";
import { NewProgram } from "./program/NewProgram";
import { ProgramBuilder } from "./program/builder/ProgramBuilder";
import { ProgramRunner } from "./program/runner/ProgramRunner";
import { NotFound } from "./NotFound";
import { ApiError } from "./api";
import { Login } from "./login/Login";

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

const RoutingApp: Component = () => {
  return (
    <Routes>
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
          path="program/new"
          component={NewProgram}
        />
        <Route
          path="program/:programId"
          component={ProgramBuilder}
        />
      </Route>
      <Route
        path="/profile/:profileId/program/:programId/run"
        component={ProgramRunner}
      />
      <Route
        path="*"
        component={NotFound}
      />
    </Routes>
  );
};

export const App: Component = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <Router>
        <RoutingApp />
      </Router>
    </QueryClientProvider>
  );
};
