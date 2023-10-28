import { Component } from "solid-js";
import { Forward } from "../../icons/Forward";
import { currentSet, day, setCurrentSet } from "./state";
import { useProgram } from "./context/ProgramProvider";
import { Select, SelectOption } from "../../forms/Select";
import { createControl } from "../../hooks/forms";
import { useParams } from "@solidjs/router";
import { createProfileQuery } from "../../hooks/queries/profiles";
import { useSwitchProfileInRunner } from "../../hooks/navigation";

const NextSet: Component<{ nSets: number }> = (props) => {
  return (
    <button
      class="text-button"
      disabled={currentSet() >= props.nSets - 1}
      onClick={() => {
        setCurrentSet((curr) => (curr >= props.nSets - 1 ? curr : curr + 1));
      }}
      title="Next Set"
    >
      <Forward />
    </button>
  );
};

const PrevSet: Component = () => {
  return (
    <button
      class="text-button"
      disabled={currentSet() === 0}
      onClick={() => {
        setCurrentSet((curr) => (curr === 0 ? curr : curr - 1));
      }}
      title="Previous Set"
    >
      <Forward class="rotate-180" />
    </button>
  );
};

const SwitchProfile: Component = () => {
  const { profileId } = useParams<{ profileId: string }>();
  const navigate = useSwitchProfileInRunner();
  const query = createProfileQuery();

  const control = createControl(profileId);

  const profileOptions = (): SelectOption[] =>
    query.data?.map((profile) => ({ value: profile.id, name: profile.name })) ?? [];

  return (
    <Select
      class="input"
      style={{
        "min-width": "5rem",
      }}
      control={control}
      options={profileOptions()}
      onChange={(event) => {
        navigate(event.target.value);
      }}
    />
  );
};

export const Tools: Component = () => {
  const { getSets } = useProgram();

  return (
    <div class="flex flex-row items-center p-1 gap-1">
      <PrevSet />
      <NextSet nSets={getSets(day()).length} />
      <SwitchProfile />
    </div>
  );
};
