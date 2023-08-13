export const days = [0, 1, 2, 3, 4, 5, 6] as const;

export type Day = (typeof days)[number];

export const dayNames = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"] as const;

export type DayName = (typeof dayNames)[Day];
