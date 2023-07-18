import { Component } from "solid-js";
import { User } from "../api";
import { A } from "@solidjs/router";
import style from "./UserCard.module.css";
import { Plus } from "../icons/Plus";

export const UserCard: Component<User> = (props) => {
  return (
    <A href={`/user/${props.id}`} class={`hover:bg-gray-600 ${style.card}`}>
      <h3 class="m-2">{props.name ?? props.username}</h3>
    </A>
  );
};

export const LoadingUserCard: Component = () => {
  return <div class={`shimmer ${style.card}`}></div>;
};

export const AddUserCard: Component = () => {
  return (
    <A href="/user/new" class={`hover:bg-gray-600 ${style.card}`}>
      <Plus />
    </A>
  );
};
