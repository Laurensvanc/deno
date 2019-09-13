// Copyright 2018-2019 the Deno authors. All rights reserved. MIT license.
import * as minimal from "./dispatch_minimal.ts";
import * as json from "./dispatch_json.ts";

// These consts are shared with Rust. Update with care.
export const OP_READ = 1;
export const OP_WRITE = 2;
export const OP_EXIT = 3;
export const OP_IS_TTY = 4;
export const OP_ENV = 5;
export const OP_EXEC_PATH = 6;
export const OP_UTIME = 7;
export const OP_SET_ENV = 8;
export const OP_HOME_DIR = 9;
export const OP_START = 10;
export const OP_APPLY_SOURCE_MAP = 11;
export const OP_FORMAT_ERROR = 12;
export const OP_CACHE = 13;
export const OP_FETCH_SOURCE_FILES = 14;
export const OP_OPEN = 15;
export const OP_CLOSE = 16;
export const OP_SEEK = 17;
export const OP_FETCH = 18;
export const OP_METRICS = 19;
export const OP_REPL_START = 20;
export const OP_REPL_READLINE = 21;
export const OP_ACCEPT = 22;
export const OP_DIAL = 23;
export const OP_SHUTDOWN = 24;
export const OP_LISTEN = 25;
export const OP_RESOURCES = 26;
export const OP_GET_RANDOM_VALUES = 27;
export const OP_GLOBAL_TIMER_STOP = 28;
export const OP_GLOBAL_TIMER = 29;
export const OP_NOW = 30;
export const OP_PERMISSIONS = 31;
export const OP_REVOKE_PERMISSION = 32;
export const OP_CREATE_WORKER = 33;
export const OP_HOST_GET_WORKER_CLOSED = 34;
export const OP_HOST_POST_MESSAGE = 35;
export const OP_HOST_GET_MESSAGE = 36;
export const OP_WORKER_POST_MESSAGE = 37;
export const OP_WORKER_GET_MESSAGE = 38;
export const OP_RUN = 39;
export const OP_RUN_STATUS = 40;
export const OP_KILL = 41;
export const OP_CHDIR = 42;
export const OP_MKDIR = 43;
export const OP_CHMOD = 44;
export const OP_CHOWN = 45;
export const OP_REMOVE = 46;
export const OP_COPY_FILE = 47;
export const OP_STAT = 48;
export const OP_READ_DIR = 49;
export const OP_RENAME = 50;
export const OP_LINK = 51;
export const OP_SYMLINK = 52;
export const OP_READ_LINK = 53;
export const OP_TRUNCATE = 54;
export const OP_MAKE_TEMP_DIR = 55;
export const OP_CWD = 56;
export const OP_FETCH_ASSET = 57;
=======
export const OP_READ = 1001;
export const OP_WRITE = 1002;
export const OP_EXIT = 2003;
export const OP_IS_TTY = 2004;
export const OP_ENV = 2005;
export const OP_EXEC_PATH = 2006;
export const OP_UTIME = 2007;
export const OP_SET_ENV = 2008;
export const OP_HOME_DIR = 2009;
export const OP_START = 2010;
export const OP_APPLY_SOURCE_MAP = 2011;
export const OP_FORMAT_ERROR = 2012;
export const OP_CACHE = 2013;
export const OP_FETCH_SOURCE_FILE = 2014;
export const OP_OPEN = 2015;
export const OP_CLOSE = 2016;
export const OP_SEEK = 2017;
export const OP_FETCH = 2018;
export const OP_METRICS = 2019;
export const OP_REPL_START = 2020;
export const OP_REPL_READLINE = 2021;
export const OP_ACCEPT = 2022;
export const OP_DIAL = 2023;
export const OP_SHUTDOWN = 2024;
export const OP_LISTEN = 2025;
export const OP_RESOURCES = 2026;
export const OP_GET_RANDOM_VALUES = 2027;
export const OP_GLOBAL_TIMER_STOP = 2028;
export const OP_GLOBAL_TIMER = 2029;
export const OP_NOW = 2030;
export const OP_PERMISSIONS = 2031;
export const OP_REVOKE_PERMISSION = 2032;
export const OP_CREATE_WORKER = 2033;
export const OP_HOST_GET_WORKER_CLOSED = 2034;
export const OP_HOST_POST_MESSAGE = 2035;
export const OP_HOST_GET_MESSAGE = 2036;
export const OP_WORKER_POST_MESSAGE = 2037;
export const OP_WORKER_GET_MESSAGE = 2038;
export const OP_RUN = 2039;
export const OP_RUN_STATUS = 2040;
export const OP_KILL = 2041;
export const OP_CHDIR = 2042;
export const OP_MKDIR = 2043;
export const OP_CHMOD = 2044;
export const OP_CHOWN = 2045;
export const OP_REMOVE = 2046;
export const OP_COPY_FILE = 2047;
export const OP_STAT = 2048;
export const OP_READ_DIR = 2049;
export const OP_RENAME = 2050;
export const OP_LINK = 2051;
export const OP_SYMLINK = 2052;
export const OP_READ_LINK = 2053;
export const OP_TRUNCATE = 2054;
export const OP_MAKE_TEMP_DIR = 2055;
export const OP_CWD = 2056;
export const OP_FETCH_ASSET = 2057;
=======
export let OP_READ: number;
export let OP_WRITE: number;
export let OP_EXIT: number;
export let OP_IS_TTY: number;
export let OP_ENV: number;
export let OP_EXEC_PATH: number;
export let OP_UTIME: number;
export let OP_SET_ENV: number;
export let OP_HOME_DIR: number;
export let OP_START: number;
export let OP_APPLY_SOURCE_MAP: number;
export let OP_FORMAT_ERROR: number;
export let OP_CACHE: number;
export let OP_FETCH_SOURCE_FILE: number;
export let OP_OPEN: number;
export let OP_CLOSE: number;
export let OP_SEEK: number;
export let OP_FETCH: number;
export let OP_METRICS: number;
export let OP_REPL_START: number;
export let OP_REPL_READLINE: number;
export let OP_ACCEPT: number;
export let OP_DIAL: number;
export let OP_SHUTDOWN: number;
export let OP_LISTEN: number;
export let OP_RESOURCES: number;
export let OP_GET_RANDOM_VALUES: number;
export let OP_GLOBAL_TIMER_STOP: number;
export let OP_GLOBAL_TIMER: number;
export let OP_NOW: number;
export let OP_PERMISSIONS: number;
export let OP_REVOKE_PERMISSION: number;
export let OP_CREATE_WORKER: number;
export let OP_HOST_GET_WORKER_CLOSED: number;
export let OP_HOST_POST_MESSAGE: number;
export let OP_HOST_GET_MESSAGE: number;
export let OP_WORKER_POST_MESSAGE: number;
export let OP_WORKER_GET_MESSAGE: number;
export let OP_RUN: number;
export let OP_RUN_STATUS: number;
export let OP_KILL: number;
export let OP_CHDIR: number;
export let OP_MKDIR: number;
export let OP_CHMOD: number;
export let OP_CHOWN: number;
export let OP_REMOVE: number;
export let OP_COPY_FILE: number;
export let OP_STAT: number;
export let OP_READ_DIR: number;
export let OP_RENAME: number;
export let OP_LINK: number;
export let OP_SYMLINK: number;
export let OP_READ_LINK: number;
export let OP_TRUNCATE: number;
export let OP_MAKE_TEMP_DIR: number;
export let OP_CWD: number;
export let OP_FETCH_ASSET: number;

export function asyncMsgFromRust(opId: number, ui8: Uint8Array): void {
  switch (opId) {
    case OP_WRITE:
    case OP_READ:
      minimal.asyncMsgFromRust(opId, ui8);
      break;
    case OP_EXIT:
    case OP_IS_TTY:
    case OP_ENV:
    case OP_EXEC_PATH:
    case OP_UTIME:
    case OP_OPEN:
    case OP_SEEK:
    case OP_FETCH:
    case OP_REPL_START:
    case OP_REPL_READLINE:
    case OP_ACCEPT:
    case OP_DIAL:
    case OP_GLOBAL_TIMER:
    case OP_HOST_GET_WORKER_CLOSED:
    case OP_HOST_GET_MESSAGE:
    case OP_WORKER_GET_MESSAGE:
    case OP_RUN_STATUS:
    case OP_MKDIR:
    case OP_CHMOD:
    case OP_CHOWN:
    case OP_REMOVE:
    case OP_COPY_FILE:
    case OP_STAT:
    case OP_READ_DIR:
    case OP_RENAME:
    case OP_LINK:
    case OP_SYMLINK:
    case OP_READ_LINK:
    case OP_TRUNCATE:
    case OP_MAKE_TEMP_DIR:
      json.asyncMsgFromRust(opId, ui8);
      break;
    default:
      throw Error("bad async opId");
  }
}
