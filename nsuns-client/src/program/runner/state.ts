import { createEffect, createSignal, on } from "solid-js";
import { dayNames } from "../../util/days";

export const today = () => new Date().getDay();

export const [day, setDay] = createSignal(today());
export const [currentSet, setCurrentSet] = createSignal(0);
export const [direction, setDirection] = createSignal(0);

export const dayName = () => dayNames[day()];
export const prevDayName = () => dayNames[prevDay(day())];
export const nextDayName = () => dayNames[nextDay(day())];

export const nextDay = (day: number) => (day === 6 ? 0 : day + 1);
export const prevDay = (day: number) => (day === 0 ? 6 : day - 1);

export const incrementDay = () => {
  setDay(nextDay);
  setDirection(1);
};
export const decrementDay = () => {
  setDay(prevDay);
  setDirection(-1);
};
export const goToToday = () => {
  const t = today();
  const d = day();
  setDay(t);
  setDirection(t < d ? -1 : t === d ? 0 : 1);
};

createEffect(
  on(day, () => {
    setCurrentSet(0);
  })
);
