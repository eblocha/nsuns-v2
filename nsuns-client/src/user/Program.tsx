import { Component } from "solid-js";
import { Program } from "../api/program";
import styles from "./Program.module.css";
import { A } from "@solidjs/router";
import { Plus } from "../icons/Plus";
import { Play } from "../icons/Play";

export const ProgramItem: Component<{
  program: Program;
  index: number;
}> = (props) => {
  return (
    <A
      href={`program/${props.program.id}`}
      class={`w-full ${styles.program} hover:bg-gray-900 flex flex-row`}
    >
      <div class="w-full flex flex-row justify-between items-center">
        <span class="flex-grow">
          {props.program.name ?? `Program ${props.index}`}
        </span>
        <A href={`program/${props.program.id}/run`} class="text-button">
          <Play />
        </A>
      </div>
    </A>
  );
};

export const LoadingProgram: Component = () => {
  return <div class={`${styles.program} shimmer h-10 w-full`}></div>;
};

export const AddProgram: Component = () => {
  return (
    <A href="program/new" class={`hover:bg-gray-700 ${styles.program}`}>
      <div class="flex flex-row items-center justify-start">
        <Plus />
        <span class="mx-4">Create New</span>
      </div>
    </A>
  );
};
