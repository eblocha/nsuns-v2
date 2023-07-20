import { useParams } from "@solidjs/router";
import { createQuery } from "@tanstack/solid-query";
import { Component } from "solid-js";

export const ProgramBuilder: Component = () => {
  const params = useParams<{ userId: string; programId: string }>();


  return <div class="w-full h-full overflow-hidden">

  </div>
};
