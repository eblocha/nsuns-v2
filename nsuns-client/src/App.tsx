import { Route, Router, Routes } from "@solidjs/router";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { Login } from "./login/Login";
import { CreateUser } from "./login/CreateUser";
import { UserHome } from "./user/UserHome";
import { NewProgram } from "./program/NewProgram";

const queryClient = new QueryClient();

export const App: Component = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <Router>
        <Routes>
          <Route path="/" component={Login} />
          <Route path="/user/new" component={CreateUser} />
          <Route path="/user/:userId" component={UserHome}>
            <Route path="/" />
            <Route path="program/new" component={NewProgram} />
            <Route path="program/:programId" element={<>id</>} />
          </Route>
        </Routes>
      </Router>
    </QueryClientProvider>
  );
};
