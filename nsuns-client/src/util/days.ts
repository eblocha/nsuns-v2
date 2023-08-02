export type Day = 0 | 1 | 2 | 3 | 4 | 5 | 6;

export const dayNames = [
  "Sunday",
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
] as const;

export type DayName = typeof dayNames[Day];

export const days = dayNames.map((_, i) => i) as Day[];
