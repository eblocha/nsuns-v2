import { Accessor, createEffect, createSignal, on, onCleanup } from "solid-js";

/**
 * @param isLoading The signal to determine if the fetch is occurring
 * @param delay Minimum time to show the fetch delay
 * @returns A new signal with the delays applied, to be used in the template.
 */
export const createMinimumAsyncDelay = (isLoading: Accessor<boolean>, delay: number = SHIMMER_DELAY_MS) => {
  return createAsymmetricDelay(isLoading, undefined, delay);
};

/**
 * @param isLoading The signal to determine if the async action is occurring
 * @param delayBeforeAsync How long to wait for the async action to complete before we actually show the loading state.
 * If the mutation is faster than this time, we never transition into a loading state, and the user sees a synchronous experience.
 * @param delay Minimum time to show the loading state if we transitioned into showing the loading state at all.
 * This will prevent UI flash if the async action finishes quickly after transitioning into the loading state.
 * @returns A new signal with the delays applied, to be used in the template.
 */
export const createSmartAsyncDelay = (
  isLoading: Accessor<boolean>,
  delayBeforeAsync: number = DELAY_BEFORE_ASYNC_MS,
  delay: number = SHIMMER_DELAY_MS
) => {
  return createAsymmetricDelay(isLoading, delayBeforeAsync, delay);
};

/**
 * Create an asymmetric delay on a signal changing value
 * @param signal The signal to operate on
 * @param delayHigh The delay to apply to the signal becoming `true`
 * @param delayLow The delay to apply to the signal becoming `false`
 * @returns A new signal with the delays applied.
 */
export const createAsymmetricDelay = (signal: Accessor<boolean>, delayHigh?: number, delayLow?: number) => {
  const [delayed, setDelayed] = createSignal(signal());

  createEffect(
    on(signal, (value) => {
      if (value) {
        if (delayHigh || value !== delayed()) {
          const timeout = setTimeout(() => setDelayed(() => value), delayHigh);
          onCleanup(() => clearTimeout(timeout));
        } else {
          setDelayed(() => value);
        }
      } else {
        if (delayLow || value !== delayed()) {
          const timeout = setTimeout(() => setDelayed(() => value), delayLow);
          onCleanup(() => clearTimeout(timeout));
        } else {
          setDelayed(() => value);
        }
      }
    })
  );

  return delayed;
};

/**
 * The amount of time the "shimmer" animation takes to cross an element.
 */
export const SHIMMER_DELAY_MS = 350;

/**
 * The recommended minimum amount of time to show a spinner, to avoid UI flash.
 */
export const SPINNER_DELAY_MS = 200;

/**
 * The recommended amount of time for an async action to complete before showing a loading state.
 */
export const DELAY_BEFORE_ASYNC_MS = 100;
