// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
import { core } from "../core.ts";

export function loadavg(): number[] {
  return core.dispatchJson.sendSync("op_loadavg");
}

export function hostname(): string {
  return core.dispatchJson.sendSync("op_hostname");
}

export function osRelease(): string {
  return core.dispatchJson.sendSync("op_os_release");
}

export function exit(code = 0): never {
  core.dispatchJson.sendSync("op_exit", { code });
  throw new Error("Code not reachable");
}

function setEnv(key: string, value: string): void {
  core.dispatchJson.sendSync("op_set_env", { key, value });
}

function getEnv(key: string): string | undefined {
  return core.dispatchJson.sendSync("op_get_env", { key })[0];
}

function deleteEnv(key: string): void {
  core.dispatchJson.sendSync("op_delete_env", { key });
}

export const env = {
  get: getEnv,
  toObject(): { [key: string]: string } {
    return core.dispatchJson.sendSync("op_env");
  },
  set: setEnv,
  delete: deleteEnv,
};

export function execPath(): string {
  return core.dispatchJson.sendSync("op_exec_path");
}
