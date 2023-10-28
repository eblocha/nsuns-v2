import { Route, Router, Routes } from "@solidjs/router";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { Login } from "./login/Login";
import { CreateProfile } from "./login/CreateProfile";
import { ProfileHome } from "./profile/ProfileHome";
import { NewProgram } from "./program/NewProgram";
import { ProgramBuilder } from "./program/builder/ProgramBuilder";
import { ProgramRunner } from "./program/runner/ProgramRunner";
import { NotFound } from "./NotFound";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      staleTime: 200,
    },
  },
});

export const App: Component = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <Router>
        <Routes>
          <Route
            path="/"
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
      </Router>
    </QueryClientProvider>
  );
};
