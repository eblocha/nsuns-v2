import { Accessor, createEffect, createSignal, on, onCleanup } from "solid-js";

/**
 * Delay a signal becoming falsy. Set `invert` true to delay becoming truthy.
 * This will ensure the signal is truthy for at least `delay` ms
 */
export const createDelayedLatch = <T>(
  signal: Accessor<T>,
  delay: number,
  invert?: boolean
) => {
  const [delayed, setDelayed] = createSignal(signal());

  let timeout: ReturnType<typeof setTimeout>;
  let turnedTrue = performance.now();

  createEffect(
    on(signal, (value) => {
      const now = performance.now();
      const elapsed = Math.floor(now - turnedTrue);
      if (value || invert || elapsed > delay) {
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
