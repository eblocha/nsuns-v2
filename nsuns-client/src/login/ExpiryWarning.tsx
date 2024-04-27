import { Component, Show } from "solid-js";
import { useUserInfoQuery } from "../hooks/queries/auth";

export const ExpiryWarning: Component = () => {
  const userInfo = useUserInfoQuery();

  const expiry = () => {
    if (userInfo.data?.type !== "anonymous") {
      return "";
    }

    const dt = new Date(userInfo.data.expiryDate);
    return dt.toDateString() + " at " + dt.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" });
  };

  return (
    <Show when={userInfo.isSuccess && userInfo.data.type === "anonymous"}>
      <div class="absolute bottom-0 right-0 m-5 rounded border border-gray-500 p-3 bg-gray-900">
        Your free trial period will end on {expiry()}.
      </div>
    </Show>
  );
};
