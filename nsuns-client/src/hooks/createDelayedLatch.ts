import { Accessor, createEffect, createSignal, on, onCleanup } from "solid-js";

/**
 * Delay a signal becoming falsy. Set `invert` true to delay becoming truthy.
 * This will ensure the signal is truthy for at least `delay` ms
 */
export const createDelayedLatch = <T>(signal: Accessor<T>, delay: number, invert?: boolean) => {
  const [delayed, setDelayed] = createSignal(signal());

  let timeout: ReturnType<typeof setTimeout>;
  let turnedTrue = performance.now();

  createEffect(
    on(signal, (value) => {
      const now = performance.now();
      const elapsed = Math.floor(now - turnedTrue);

      const isImmediateValue = invert ? !value : !!value;

      if (isImmediateValue || elapsed > delay) {
        setDelayed(() => value);
        turnedTrue = now;
      } else {
        timeout = setTimeout(() => setDelayed(() => value), delay - elapsed);
        onCleanup(() => clearTimeout(timeout));
      }
    })
  );

  return delayed;
};

/**
 * A small amount of delay added to `isLoading` states turning false to prevent UI flash.
 * This effectively creates a "minimum" time that async actions appear to take in the UI. If they are slower than this,
 * this time is not added on top.
 * 
 * It is set to the ammount of time the "shimmer" animation takes to traverse elements.
 */
export const SHIMMER_DELAY_MS = 350;
