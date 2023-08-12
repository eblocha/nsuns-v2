import { Component } from "solid-js";
import { Profile } from "../api";
import { A } from "@solidjs/router";
import style from "./ProfileCard.module.css";
import { Plus } from "../icons/Plus";

export const ProfileCard: Component<Profile> = (props) => {
  return (
    <A
      href={`/profile/${props.id}`}
      class={`hover:bg-gray-600 ${style.card}`}
    >
      <h3 class="m-2">{props.name}</h3>
    </A>
  );
};

export const LoadingProfileCard: Component = () => {
  return <div class={`shimmer ${style.card}`}></div>;
};

export const AddProfileCard: Component = () => {
  return (
    <A
      href="/profile/new"
      class={`hover:bg-gray-600 ${style.card}`}
    >
      <Plus />
    </A>
  );
};
