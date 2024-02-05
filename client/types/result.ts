export type apiResult<T, E = string> =
  | { type: "INIT" }
  | { type: "LOADING" }
  | { type: "LOADED"; data: T }
  | { type: "ERROR"; err: E; code?: number; errMsg?: string };
export type result<T, E = string> =
  | {
      type: "OK";
      data: T;
    }
  | {
      type: "ERR";
      data: E;
    };
