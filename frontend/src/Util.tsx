import React from 'react';

export type Error = string;
type Mode = "unloaded"|"loading" | "loaded" | "error";

export class Loadable<T> {
  mode: Mode;
  value: null | T | Error;

  constructor() {
    this.mode = "loading";
    this.value = null;
  }
  static unloaded<T>(): Loadable<T> {
    let loadable = new Loadable<T>();
    loadable.mode = "unloaded";
    loadable.value = null;
    return loadable;
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

  getLoaded(): T | null {
    if (this.mode == "loaded")  {
      return this.value as any as T
    } else {
      return null
    }
  }

  match(options: LoadableMatchOptions<T> ): JSX.Element {
    switch (this.mode) {
      case "unloaded":
        if (options.unloaded !== undefined){
          return options.unloaded()
        } else {
          return (<></>)
        }
      case "loading":
        return options.loading();
      case "loaded":
        return options.loaded(this.value as any as T);
      case "error":
        return options.error(this.value as any as Error);
      default:
        return <></>
    }
  }
}

export interface LoadableMatchOptions<T> {
  unloaded?: () => JSX.Element;
  loading: () => JSX.Element;
  loaded: (value: T) => JSX.Element;
  error: (error: Error) => JSX.Element;
}



