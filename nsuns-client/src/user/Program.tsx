import { Component } from "solid-js";
import { Program } from "../api/program";
import styles from "./Program.module.css";
import { A } from "@solidjs/router";
import { Plus } from "../icons/Plus";

export const ProgramItem: Component<{
  program: Program;
  index: number;
  isDefault?: boolean;
}> = (props) => {
  return (
    <A
      href={`/programs/${props.program.id}`}
      class={`w-full ${styles.program}`}
      classList={{
        [styles.default]: props.isDefault,
      }}
    >
      {props.program.name ?? `Program ${props.index}`}
    </A>
  );
};

export const LoadingProgram: Component = () => {
  return <div class={`${styles.program} shimmer h-10 w-full`}></div>;
};

export const AddProgram: Component = () => {
  return (
    <A href="create-program" class={`hover:bg-gray-700 ${styles.program}`}>
      <div class="flex flex-row items-center justify-start">
        <Plus />
        <span class="mx-4">Create New</span>
      </div>
    </A>
  );
};
