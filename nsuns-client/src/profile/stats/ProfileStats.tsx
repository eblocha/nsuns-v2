import { Component } from "solid-js";
import { StatsProvider } from "../../stats/StatsProvider";
import { UpdateMaxes } from "./UpdateMaxes";
import { StatList } from "./maxes/StatList";
import { AddMax } from "./maxes/AddMax";
import { UndoUpdate } from "./UndoUpdate";

export const ProfileStats: Component<{ profileId: string }> = (props) => {
  return (
    <StatsProvider profileId={props.profileId}>
      <div class="w-full flex flex-col gap-4 mt-8">
          <h3 class="text-xl">Your Stats</h3>
          <StatList />
          <div class="flex flex-row items-stretch gap-2 flex-wrap">
            <AddMax />
            <UpdateMaxes />
            <UndoUpdate />
          </div>
        </div>
    </StatsProvider>
  )
};
