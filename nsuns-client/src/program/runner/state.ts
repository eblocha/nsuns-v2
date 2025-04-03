import { createEffect, createSignal, on } from "solid-js";
import { Day } from "../../util/days";

export const [today, setToday] = createSignal(new Date().getDay() as Day);

export const [day, setDay] = createSignal(today());
export const [currentSet, setCurrentSet] = createSignal(0);
export const [direction, setDirection] = createSignal<-1 | 0 | 1>(0);

export const nextDay = (day: number) => (day === 6 ? 0 : day + 1) as Day;
export const prevDay = (day: number) => (day === 0 ? 6 : day - 1) as Day;

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

setInterval(() => {
  // Update the current day every minute
  setToday(new Date().getDay() as Day);
}, 60_000);
