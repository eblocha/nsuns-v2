import { Component } from "solid-js";
import { Edit } from "../icons/Edit";

export const Movement: Component<{
  name: string;
  description?: string | null;
}> = (props) => {
  return (
    <li
      class="border rounded border-gray-400 p-2 my-1 flex flex-row items-center justify-between"
    >
      <div>
        <p>{props.name}</p>
        <p class="text-sm opacity-80">{props.description}</p>
      </div>
      <button class="text-button">
        <Edit class="text-gray-300" />
      </button>
    </li>
  );
};
