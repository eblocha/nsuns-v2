import { Component, Show, createEffect, createSignal } from "solid-js";
import { useUserInfoQuery } from "../hooks/queries/auth";
import { ChevronLeft } from "../icons/ChevronLeft";

export const ExpiryWarning: Component = () => {
  const [isVisible, setVisible] = createSignal(false);

  const userInfo = useUserInfoQuery();

  const expiry = () => {
    if (userInfo.data?.type !== "anonymous") {
      return "";
    }

    const dt = new Date(userInfo.data.expiryDate);
    return dt.toDateString() + " at " + dt.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" });
  };

  const isAnonymous = () => userInfo.isSuccess && userInfo.data.type === "anonymous";

  createEffect(() => {
    if (isAnonymous()) {
      setVisible(true);
    }
  });

  return (
    <Show when={isAnonymous()}>
      <Show
        when={isVisible()}
        fallback={
          <button
            class="absolute bottom-0 right-0 m-5 secondary-button"
            onClick={() => setVisible(true)}
          >
            <ChevronLeft />
          </button>
        }
      >
        <div class="absolute bottom-0 right-0 m-5 rounded border border-gray-500 p-3 bg-gray-900 flex flex-row items-center gap-2">
          <button
            class="text-button"
            onClick={() => setVisible(false)}
          >
            <ChevronLeft class="rotate-180" />
          </button>
          Your free trial period will end on {expiry()}.
        </div>
      </Show>
    </Show>
  );
};
