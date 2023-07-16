import { Component } from "solid-js";
import { User } from "../api";
import { A } from "@solidjs/router";
import style from "./UserCard.module.css";

export const UserCard: Component<User> = (props) => {
  return (
    <A href={`/user/${props.id}`} class={`hover:bg-blue-100 ${style.card}`}>
      <h3 class="m-2">{props.name ?? props.username}</h3>
    </A>
  );
};

export const LoadingUserCard: Component = (props) => {
  return (
    <div class={`${style.shimmer} ${style.card}`}>
    </div>
  );
};
