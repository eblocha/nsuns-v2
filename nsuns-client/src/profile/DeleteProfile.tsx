import { Component, Show } from "solid-js";
import { Profile } from "../api";
import { useNavigate } from "@solidjs/router";
import { createDeleteProfileMutation } from "../hooks/queries/profiles";
import { Modal } from "../modal/Modal";
import { Spinner } from "../icons/Spinner";

export const DeleteProfile: Component<{
  show: boolean;
  close: () => void;
  profile: Profile;
}> = (props) => {
  const navigate = useNavigate();

  const mutation = createDeleteProfileMutation({
    onSuccess: () => {
      props.close();
      navigate("/");
    },
  });

  return (
    <Modal open={props.show} onBackdropClick={props.close}>
      <div class="bg-gray-900 p-8 rounded" onClick={(e) => e.stopPropagation()}>
        <p>Permanently delete profile: {props.profile.name}?</p>
        <div class="grid grid-cols-2 mt-4 ml-auto">
          <button
            class="secondary-button"
            disabled={mutation.isLoading}
            onClick={props.close}
          >
            Cancel
          </button>
          <button
            class="danger-button ml-2 flex flex-row items-center justify-center"
            disabled={mutation.isLoading}
            onClick={() => {
              mutation.mutate(props.profile.id);
            }}
          >
            <Show
              when={!mutation.isLoading}
              fallback={<Spinner class="animate-spin" />}
            >
              Delete
            </Show>
          </button>
        </div>
      </div>
    </Modal>
  );
};
