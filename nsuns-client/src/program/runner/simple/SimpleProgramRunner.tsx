import { A, useParams } from "@solidjs/router";
import { Component } from "solid-js";
import { ProgramProvider } from "../context/ProgramProvider";
import { User } from "../../../icons/User";
import { Edit } from "../../../icons/Edit";
import { TitleBanner } from "./SetTitle";
import { Tools } from "./Tools";
import { DataList } from "./data/DataList";

export const SimpleProgramRunner: Component = () => {
  const params = useParams<{ programId: string; profileId: string }>();

  return (
    <ProgramProvider
      profileId={params.profileId}
      programId={params.programId}
    >
      <div class="w-full h-full flex-shrink-0 flex flex-col gap-2">
        <div class="w-full flex-grow flex flex-row">
          <nav class="flex flex-col items-center p-2 flex-shrink-0 gap-2">
            <A
              href="/"
              class="text-button text-2xl"
            >
              <User />
            </A>
            <A
              href="../"
              class="text-button text-2xl"
            >
              <Edit />
            </A>
          </nav>
          <div class="p-6 flex-grow flex flex-col gap-8">
            <div class="flex-grow overflow-auto">
              <TitleBanner />
            </div>
          </div>
          <div class="flex-shrink-0 p-6 flex flex-col gap-4 w-60">
            <DataList />
          </div>
        </div>
        <div class="mt-auto p-6">
          <Tools />
        </div>
      </div>
    </ProgramProvider>
  );
};
