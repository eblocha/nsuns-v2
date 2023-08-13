import { it, expect, describe } from "vitest";
import { Section, getSections, plural, repsDisplay, resolvedWeightDisplay, round } from "./setDisplay";
import { Movement, ProgramSet } from "../api";

const baseProgramSet: ProgramSet = {
  id: "",
  amount: 0,
  day: 0,
  description: null,
  movementId: "",
  percentageOfMax: null,
  programId: "",
  reps: null,
  repsIsMinimum: false,
};

describe("plural", () => {
  it("gives plural suffix for n = 0", () => {
    expect(plural(0)).toBe("s");
  });

  it("gives no suffix for n = 1", () => {
    expect(plural(1)).toBe("");
  });

  it("gives plural suffix for n > 1", () => {
    expect(plural(2)).toBe("s");
  });
});

describe("repsDisplay", () => {
  type Case = {
    programSet: ProgramSet;
    expected: string;
    description: string;
  };

  const each = it.each<Case>([
    {
      programSet: baseProgramSet,
      expected: "",
      description: "reps is null",
    },
    {
      programSet: { ...baseProgramSet, reps: 0 },
      expected: " for 0 reps",
      description: "zero reps",
    },
    {
      programSet: { ...baseProgramSet, reps: 1 },
      expected: " for 1 rep",
      description: "one rep",
    },
    {
      programSet: { ...baseProgramSet, reps: 5 },
      expected: " for 5 reps",
      description: "five reps",
    },
    {
      programSet: { ...baseProgramSet, reps: 1, repsIsMinimum: true },
      expected: " for 1+ reps",
      description: "one rep minimum",
    },
    {
      programSet: { ...baseProgramSet, reps: 0, repsIsMinimum: true },
      expected: " for 0+ reps",
      description: "zero rep minimum",
    },
    {
      programSet: { ...baseProgramSet, reps: 5, repsIsMinimum: true },
      expected: " for 5+ reps",
      description: "five rep minimum",
    },
  ]);

  each("$description => $expected", ({ programSet, expected }) => {
    expect(repsDisplay(programSet)).toBe(expected);
  });
});

describe("round", () => {
  type Case = {
    amount: number;
    expected: number;
  };
  it.each<Case>([
    { amount: 10, expected: 10 },
    { amount: 3, expected: 5 },
    { amount: 3.2, expected: 5 },
    { amount: 2, expected: 0 },
    { amount: 7, expected: 5 },
  ])("rounds $amount to $expected", ({ amount, expected }) => {
    expect(round(amount)).toBe(expected);
  });
});

describe("resolvedWeightDisplay", () => {
  type Case = {
    programSet: ProgramSet;
    max: number | undefined;
    expected: string;
    description: string;
  };

  it.each<Case>([
    {
      programSet: baseProgramSet,
      max: undefined,
      expected: "",
      description: "Zero amount, non-pct set",
    },
    {
      programSet: {
        ...baseProgramSet,
        amount: 20,
      },
      max: undefined,
      expected: " 20 lbs",
      description: "20 lbs, non-pct set",
    },
    {
      programSet: {
        ...baseProgramSet,
        amount: 1,
      },
      max: undefined,
      expected: " 1 lb",
      description: "1 lb, non-pct set",
    },
    {
      programSet: {
        ...baseProgramSet,
        amount: 20,
        percentageOfMax: "id",
      },
      max: 100,
      expected: " 20 lbs",
      description: "20 pct of 100 lbs",
    },
    {
      programSet: {
        ...baseProgramSet,
        amount: 20,
        percentageOfMax: "id",
      },
      max: 15,
      expected: " 5 lbs",
      description: "20 pct of 15 lbs",
    },
    {
      programSet: {
        ...baseProgramSet,
        amount: 20,
        percentageOfMax: "id",
      },
      max: undefined,
      expected: "",
      description: "20 pct of unspecified",
    },
  ])("$description => $expected", ({ programSet, max, expected }) => {
    expect(resolvedWeightDisplay(programSet, max)).toBe(expected);
  });
});

describe("getSections", () => {
  type Case = {
    sets: ProgramSet[];
    movements: Record<string, Movement>;
    expected: Section[];
    description: string;
  };

  const movement1: Movement = {
    description: "",
    id: "1",
    name: "",
  };

  const movement2: Movement = {
    description: "",
    id: "2",
    name: "",
  };

  const setForM1: ProgramSet = {
    ...baseProgramSet,
    movementId: movement1.id,
  };

  const setForM2: ProgramSet = {
    ...baseProgramSet,
    movementId: movement2.id,
  };

  it.each<Case>([
    {
      sets: [],
      movements: {},
      expected: [],
      description: "empty",
    },
    {
      sets: [setForM1, setForM1, setForM2, setForM1],
      movements: {
        [movement1.id]: movement1,
        [movement2.id]: movement2,
      },
      expected: [
        {
          movement: movement1,
          sets: [
            {
              set: setForM1,
              index: 0,
            },
            {
              set: setForM1,
              index: 1,
            },
          ],
        },
        {
          movement: movement2,
          sets: [
            {
              set: setForM2,
              index: 2,
            },
          ],
        },
        {
          movement: movement1,
          sets: [
            {
              set: setForM1,
              index: 3,
            },
          ],
        },
      ],
      description: "grouping",
    },
    {
      sets: [setForM1, setForM1, setForM2, setForM1],
      movements: {
        [movement1.id]: movement1,
      },
      description: "dangling movement ref",
      expected: [
        {
          movement: movement1,
          sets: [
            {
              set: setForM1,
              index: 0,
            },
            {
              set: setForM1,
              index: 1,
            },
            {
              set: setForM1,
              // index skips over because it is a reference into the original ProgramSet[] array
              index: 3,
            },
          ],
        },
      ],
    },
  ])("$description", ({ sets, movements, expected }) => {
    expect(getSections(sets, movements)).toStrictEqual(expected);
  });
});
