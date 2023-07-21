import { Accessor, Setter, batch, createMemo, createSignal } from "solid-js";

export type ErrorInfo = Record<string, boolean>;

export type Validator<T> = (value: T) => ErrorInfo;

export type InputOptions<T> = {
  validators?: Validator<T>[];
  errorMessageMapping?: Record<string, string>;
};

export const DEFAULT_ERROR_MESSAGES = {
  isMissing: "This field is required.",
};

export type Control<T> = {
  value: Accessor<T>;
  setValue: Setter<T>;
  dirty: Accessor<boolean>;
  setDirty: Setter<boolean>;
  touched: Accessor<boolean>;
  setTouched: Setter<boolean>;
  errors: Accessor<ErrorInfo>;
  errorMessages: Accessor<string[]>;
  hasErrors: Accessor<boolean>;
  showErrors: Accessor<boolean>;
  reset: (value?: T) => void;
};

export const required =
  <T>(): Validator<T> =>
  (value) => {
    return {
      isMissing: !value,
    };
  };

export const createControl = <T>(
  initialValue: T,
  options?: InputOptions<T>
): Control<T> => {
  const [value, setValue] = createSignal(initialValue);
  const [dirty, setDirty] = createSignal(false);
  const [touched, setTouched] = createSignal(false);

  const errorMessageMapping: Record<string, string> =
    options?.errorMessageMapping ?? DEFAULT_ERROR_MESSAGES;

  const errors = createMemo(() => {
    const current = value();
    const errorMap: ErrorInfo = {};

    options?.validators?.forEach((validator) => {
      const errs = validator(current);
      for (const key in errs) {
        errorMap[key] = errs[key];
      }
    });
    return errorMap;
  });

  const errorMessages = () =>
    Object.entries(errors())
      .map(([name, isErrored]) =>
        isErrored ? errorMessageMapping[name] : undefined
      )
      .filter((value): value is string => !!value);

  const hasErrors = createMemo(() => !Object.values(errors()).every((v) => !v));

  const showErrors = () => dirty() && hasErrors();

  const reset = (value?: T) =>
    batch(() => {
      setDirty(false);
      setTouched(false);
      setValue(() => (value === undefined ? initialValue : value));
    });

  return {
    value,
    setValue,
    dirty,
    setDirty,
    touched,
    setTouched,
    errors,
    errorMessages,
    hasErrors,
    showErrors,
    reset,
  };
};

type ControlValue<C> = C extends Control<infer V> ? V : never;

type ControlValues<R extends Record<string, Control<unknown>>> = {
  [K in keyof R]: ControlValue<R[K]>;
};

export const createControlGroup = <R extends Record<string, Control<any>>>(
  controls: R
) => {
  const dirty = createMemo(
    () => !Object.values(controls).every((control) => !control.dirty())
  );
  const touched = createMemo(
    () => !Object.values(controls).every((control) => !control.touched())
  );

  const errors = createMemo(() => {
    const errMap: Record<string, ErrorInfo> = {};
    for (const key in controls) {
      errMap[key] = controls[key].errors();
    }
  });

  const hasErrors = createMemo(
    () => !Object.values(controls).every((control) => !control.hasErrors())
  );

  const showErrors = createMemo(
    () => !Object.values(controls).every((control) => control.showErrors())
  );

  const value = () => {
    const value: ControlValues<R> = {} as ControlValues<R>;
    for (const key in controls) {
      value[key] = controls[key].value() as ControlValue<
        R[Extract<keyof R, string>]
      >;
    }
    return value;
  };

  const reset = (values?: Partial<ControlValues<R>>) =>
    batch(() =>
      Object.entries(controls).forEach(([key, control]) =>
        control.reset(values?.[key])
      )
    );

  return {
    controls,
    dirty,
    touched,
    errors,
    hasErrors,
    showErrors,
    value,
    reset,
  };
};