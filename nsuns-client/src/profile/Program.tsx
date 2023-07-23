import { Component, Show, createSignal } from "solid-js";
import { Program } from "../api/program";
import styles from "./Program.module.css";
import { A, useParams } from "@solidjs/router";
import { Plus } from "../icons/Plus";
import { Play } from "../icons/Play";
import { Trash } from "../icons/Trash";
import { Modal } from "../modal/Modal";
import { Spinner } from "../icons/Spinner";
import { useNavigateToProfileHome } from "../hooks/navigation";
import { useDeleteProgram } from "../hooks/queries/programs";

const DeleteProgram: Component<{
  show: boolean;
  close: () => void;
  program: Program;
}> = (props) => {
  const params = useParams<{ programId?: string, profileId: string }>();
  const navigateToProfileHome = useNavigateToProfileHome();

  const mutation = useDeleteProgram({
    onSuccess: () => {
      props.close();
      if (params.programId === props.program.id) {
        navigateToProfileHome();
      }
    },
  });

  return (
    <Modal open={props.show} onBackdropClick={props.close}>
      <div class="bg-gray-900 p-8 rounded" onClick={(e) => e.stopPropagation()}>
        <p>Delete Program: {props.program.name}?</p>
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
              mutation.mutate(props.program.id);
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

export const ProgramItem: Component<{
  program: Program;
  index: number;
}> = (props) => {
  const [showDeleteModal, setShowDeleteModal] = createSignal(false);

  return (
    <>
      <A
        href={`program/${props.program.id}`}
        class={`w-full ${styles.program} hover:bg-gray-900 flex flex-row`}
      >
        <div class="w-full flex flex-row justify-between items-center">
          <span class="flex-grow">
            {props.program.name ?? `Program ${props.index}`}
          </span>
          <button
            class="text-button mr-2 hover:text-red-500 focus:text-red-500 hover:transition-colors focus:transition-colors"
            onClick={(e) => {
              e.preventDefault();
              setShowDeleteModal(true);
            }}
          >
            <Trash />
          </button>
          <A href={`program/${props.program.id}/run`} class="text-button">
            <Play />
          </A>
        </div>
      </A>
      <DeleteProgram
        show={showDeleteModal()}
        close={() => setShowDeleteModal(false)}
        program={props.program}
      />
    </>
  );
};

export const LoadingProgram: Component = () => {
  return <div class={`${styles.program} shimmer h-10 w-full`}></div>;
};

export const AddProgram: Component = () => {
  return (
    <A href="program/new" class="text-button-outline">
      <div class="flex flex-row items-center justify-start gap-2">
        <Plus />
        <span>New Program</span>
      </div>
    </A>
  );
};
