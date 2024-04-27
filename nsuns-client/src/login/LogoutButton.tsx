import { Component, JSX, Show, createSignal } from "solid-js";
import { Logout } from "../icons/Logout";
import { Modal } from "../modal/Modal";
import { Spinner } from "../icons/Spinner";
import { useLogoutMutation, useUserInfoQuery } from "../hooks/queries/auth";

export const LogoutButton: Component<{ children?: JSX.Element }> = (props) => {
  const [modalOpen, setModalOpen] = createSignal(false);
  const userInfo = useUserInfoQuery();
  const mutation = useLogoutMutation();

  const isKnownAnonymous = () => userInfo.data?.type === "anonymous";

  const onClick = () => {
    // Safeguard against errors when checking whether the user is anonymous.
    // If we're not sure, open the modal.
    if (isKnownAnonymous() || userInfo.isError) {
      setModalOpen(true);
    } else if (userInfo.data?.type === "user") {
      mutation.mutate();
    }
  };

  return (
    <>
      <button
        class="text-button flex flex-row items-center gap-2 text-base whitespace-nowrap"
        title="Log out"
        aria-label="Log out"
        onClick={() => onClick()}
        disabled={userInfo.isLoading || mutation.isLoading}
      >
        <Show
          when={!userInfo.isLoading && !mutation.isLoading}
          fallback={<Spinner class="animate-spin" />}
        >
          <Logout class="text-white" />
        </Show>
        {props.children}
      </button>
      <Modal
        open={modalOpen()}
        onBackdropClick={() => setModalOpen(false)}
      >
        <div
          class="bg-gray-900 p-8 rounded max-w-md"
          onClick={(e) => e.stopPropagation()}
        >
          <h2 class="text-lg mb-2">Log out</h2>
          <Show
            when={isKnownAnonymous()}
            fallback={
              <p class="mb-2">
                We could not determine if you are a temporary user. Temporary users will lose <em>all</em> data after
                logging out.
              </p>
            }
          >
            <p class="mb-2">
              You are an temporary user. If you log out, <em>all</em> of your data will be deleted.
            </p>
          </Show>
          <p>Are you sure you want to log out?</p>
          <div class="grid grid-cols-2 mt-4 ml-auto">
            <button
              class="secondary-button"
              disabled={mutation.isLoading}
              onClick={() => setModalOpen(false)}
            >
              Cancel
            </button>
            <button
              class="danger-button ml-2 flex flex-row items-center justify-center"
              disabled={mutation.isLoading}
              onClick={() => mutation.mutate()}
            >
              <Show
                when={!mutation.isLoading}
                fallback={<Spinner class="animate-spin" />}
              >
                Yes, delete my data
              </Show>
            </button>
          </div>
        </div>
      </Modal>
    </>
  );
};
