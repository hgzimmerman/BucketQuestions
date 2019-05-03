
export type Error = string;
type Mode = "loading" | "loaded" | "error";

export class Loadable<T> {
  mode: Mode;
  value: null | T | Error;

  constructor() {
    this.mode = "loading";
    this.value = null;
  }
  static loaded<T>(value: T): Loadable<T> {
    let loadable = new Loadable<T>();
    loadable.mode = "loaded";
    loadable.value = value;
    return loadable;
  }
  static errored<T>(error: Error): Loadable<T> {
    let loadable = new Loadable<T>();
    loadable.mode = "error";
    loadable.value = error;
    return loadable;
  }
  static loading<T>(): Loadable<T> {
    return new Loadable();
  }

  match(options: LoadableMatchOptions<T> ): JSX.Element {
    switch (this.mode) {
      case "loading":
        return options.loading();
      case "loaded":
        return options.loaded(this.value as any as T);
      case "error":
        return options.error(this.value as any as Error);
    }
  }
}

export interface LoadableMatchOptions<T> {
  loading: () => JSX.Element;
  loaded: (value: T) => JSX.Element;
  error: (error: Error) => JSX.Element;
}