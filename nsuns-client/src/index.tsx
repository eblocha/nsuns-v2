/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import { Route, Router, Routes } from "@solidjs/router";
import { Login } from "./login/Login";
import { CreateUser } from "./login/CreateUser";
import { UserHome } from "./user/UserHome";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";

const root = document.getElementById("root");

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    "Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?"
  );
}

const queryClient = new QueryClient();

render(
  () => (
    <QueryClientProvider client={queryClient}>
      <Router>
        <Routes>
          <Route path="/" component={Login} />
          <Route path="/user/new" component={CreateUser} />
          <Route path="/user/:id" component={UserHome} />
        </Routes>
      </Router>
    </QueryClientProvider>
  ),
  root!
);
