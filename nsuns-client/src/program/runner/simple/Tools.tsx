import { Component, Show } from "solid-js";
import { Forward } from "../../../icons/Forward";
import { currentSet, day, decrementDay, goToToday, incrementDay, setCurrentSet, today } from "../state";
import { useProgram } from "../context/ProgramProvider";
import { Select, SelectOption } from "../../../forms/Select";
import { createControl } from "../../../hooks/forms";
import { useParams } from "@solidjs/router";
import { createProfileQuery } from "../../../hooks/queries/profiles";
import { useSwitchProfileInRunner } from "../../../hooks/navigation";
import { ArrowRight } from "../../../icons/ArrowRight";
import { dayNames } from "../../../util/days";

const NextSet: Component<{ nSets: number }> = (props) => {
  return (
    <button
      class="secondary-button w-20 h-20 flex items-center justify-center"
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
      class="secondary-button w-20 h-20 flex items-center justify-center"
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
    <div class="flex flex-row items-center gap-8">
      <PrevSet />
      <Show when={getSets(day()).length}>
        <div class="text-6xl text-gray-400">
          Set {currentSet() + 1} of {getSets(day()).length}
        </div>
      </Show>
      <NextSet nSets={getSets(day()).length} />
      <SwitchProfile />
      <div class="ml-auto flex flex-row items-center gap-8">
        <button
          class="text-button"
          onClick={goToToday}
          disabled={day() === today()}
        >
          Go To Today
        </button>
        <button
          class="w-20 h-20 circle-text-button flex flex-row items-center justify-center"
          onClick={decrementDay}
        >
          <ArrowRight class="rotate-180" />
        </button>
        <div class="text-2xl w-36 flex justify-center">{dayNames[day()]}</div>
        <button
          class="w-20 h-20 circle-text-button flex flex-row items-center justify-center"
          onClick={incrementDay}
        >
          <ArrowRight />
        </button>
      </div>
    </div>
  );
};
