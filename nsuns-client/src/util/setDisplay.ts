import { Movement, ProgramSet } from "../api";

export const plural = (value: number) => (value === 1 ? "" : "s");

/**
 * " for 3 reps", " for 1 rep", " for 1+ reps", " for 3+ reps", "" (no reps specified)
 */
export const repsDisplay = (set: ProgramSet) => {
  return set.reps != null
    ? ` for ${set.reps}${set.repsIsMinimum ? "+" : ""} rep${
        set.repsIsMinimum ? "s" : plural(set.reps)
      }`
    : "";
};

export const round = (value: number) => Math.round(value / 5) * 5;

/**
 * " 55 lbs", " 1 lb", " 0 lbs" (if percent mode), "" (if 0 amount, not percent mode)
 */
export const resolvedWeightDisplay = (set: ProgramSet, max?: number) => {
  if (set.percentageOfMax) {
    return max === undefined
      ? ""
      : ` ${round((set.amount / 100) * max).toFixed(0)} lbs`;
  } else {
    return set.amount
      ? ` ${set.amount.toFixed(0)} lb${plural(set.amount)}`
      : "";
  }
};

export type Section = {
  movement: Movement;
  sets: { set: ProgramSet; index: number }[];
};

export const getSections = (
  sets: ProgramSet[],
  movements: Record<number, Movement>
) => {
  const sections: Section[] = [];

  let currentSection: Section | null = null;
  let index = -1;
  for (const set of sets) {
    index++;
    const movement = movements[set.movementId];
    if (!movement) continue;

    if (!currentSection || currentSection.movement.id !== movement.id) {
      if (currentSection) sections.push(currentSection);

      currentSection = {
        movement: movement,
        sets: [{ set, index }],
      };
    } else if (currentSection.movement.id === movement.id) {
      currentSection.sets.push({ set, index });
    }
  }
  if (currentSection) sections.push(currentSection);

  return sections;
};
