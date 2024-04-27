import { Component, Show } from "solid-js";
import { LoginForm } from "./LoginForm";
import { loginAnonymous } from "../api/auth";
import { createMutation } from "@tanstack/solid-query";
import { useNavigateToProfileHome } from "../hooks/navigation";
import { displayError } from "../util/errors";
import { Warning } from "../icons/Warning";
import { Spinner } from "../icons/Spinner";

export const Login: Component = () => {
  // TODO add warning about logging in if user is currently anonymous, since that would delete their data
  const navigate = useNavigateToProfileHome();

  const loginAnonymousMutation = createMutation({
    mutationFn: loginAnonymous,
    onSuccess: () => {
      navigate();
    },
  });

  return (
    <div class="w-full h-full flex flex-col justify-center items-stretch p-80">
      <div class="grid grid-cols-2 w-full">
        <div class="border-r border-gray-500 p-5 flex flex-row justify-end">
          <LoginForm />
        </div>
        <div class="p-5 flex flex-col items-start gap-4">
          <h2 class="text-lg">Continue As Guest (2 day trial)</h2>
          <Show when={loginAnonymousMutation.isError}>
            <div class="w-full flex flex-row items-center gap-4">
              <span>
                <Warning class="text-red-500" />
              </span>
              {displayError(loginAnonymousMutation.error, "create session")}
            </div>
          </Show>
          <button
            class="primary-button"
            disabled={loginAnonymousMutation.isLoading}
            onClick={() => loginAnonymousMutation.mutate()}
          >
            <Show
              when={!loginAnonymousMutation.isLoading}
              fallback={<Spinner class="animate-spin my-1" />}
            >
              Continue
            </Show>
          </button>
        </div>
      </div>
    </div>
  );
};
