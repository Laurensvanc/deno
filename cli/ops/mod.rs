// Copyright 2018-2019 the Deno authors. All rights reserved. MIT license.
use crate::state::ThreadSafeState;
use deno::*;

mod compiler;
mod dispatch_json;
mod dispatch_minimal;
mod errors;
mod fetch;
mod files;
mod fs;
mod io;
mod metrics;
mod net;
mod os;
mod performance;
mod permissions;
mod process;
mod random;
mod repl;
mod resources;
mod timers;
mod workers;

use dispatch_json::{JsonDispatcher, JsonOpHandler};
use dispatch_minimal::{MinimalDispatcher, MinimalOpHandler};

// Warning! These values are duplicated in the TypeScript code (js/dispatch.ts),
// update with care.
pub const OP_READ: OpId = 1001;
pub const OP_WRITE: OpId = 1002;
pub const OP_EXIT: OpId = 2003;
pub const OP_IS_TTY: OpId = 2004;
pub const OP_ENV: OpId = 2005;
pub const OP_EXEC_PATH: OpId = 2006;
pub const OP_UTIME: OpId = 2007;
pub const OP_SET_ENV: OpId = 2008;
pub const OP_HOME_DIR: OpId = 2009;
pub const OP_START: OpId = 2010;
pub const OP_APPLY_SOURCE_MAP: OpId = 2011;
pub const OP_FORMAT_ERROR: OpId = 2012;
pub const OP_CACHE: OpId = 2013;
pub const OP_FETCH_SOURCE_FILE: OpId = 2014;
pub const OP_OPEN: OpId = 2015;
pub const OP_CLOSE: OpId = 2016;
pub const OP_SEEK: OpId = 2017;
pub const OP_FETCH: OpId = 2018;
pub const OP_METRICS: OpId = 2019;
pub const OP_REPL_START: OpId = 2020;
pub const OP_REPL_READLINE: OpId = 2021;
pub const OP_ACCEPT: OpId = 2022;
pub const OP_DIAL: OpId = 2023;
pub const OP_SHUTDOWN: OpId = 2024;
pub const OP_LISTEN: OpId = 2025;
pub const OP_RESOURCES: OpId = 2026;
pub const OP_GET_RANDOM_VALUES: OpId = 2027;
pub const OP_GLOBAL_TIMER_STOP: OpId = 2028;
pub const OP_GLOBAL_TIMER: OpId = 2029;
pub const OP_NOW: OpId = 2030;
pub const OP_PERMISSIONS: OpId = 2031;
pub const OP_REVOKE_PERMISSION: OpId = 2032;
pub const OP_CREATE_WORKER: OpId = 2033;
pub const OP_HOST_GET_WORKER_CLOSED: OpId = 2034;
pub const OP_HOST_POST_MESSAGE: OpId = 2035;
pub const OP_HOST_GET_MESSAGE: OpId = 2036;
pub const OP_WORKER_POST_MESSAGE: OpId = 2037;
pub const OP_WORKER_GET_MESSAGE: OpId = 2038;
pub const OP_RUN: OpId = 2039;
pub const OP_RUN_STATUS: OpId = 2040;
pub const OP_KILL: OpId = 2041;
pub const OP_CHDIR: OpId = 2042;
pub const OP_MKDIR: OpId = 2043;
pub const OP_CHMOD: OpId = 2044;
pub const OP_CHOWN: OpId = 2045;
pub const OP_REMOVE: OpId = 2046;
pub const OP_COPY_FILE: OpId = 2047;
pub const OP_STAT: OpId = 2048;
pub const OP_READ_DIR: OpId = 2049;
pub const OP_RENAME: OpId = 2050;
pub const OP_LINK: OpId = 2051;
pub const OP_SYMLINK: OpId = 2052;
pub const OP_READ_LINK: OpId = 2053;
pub const OP_TRUNCATE: OpId = 2054;
pub const OP_MAKE_TEMP_DIR: OpId = 2055;
pub const OP_CWD: OpId = 2056;
pub const OP_FETCH_ASSET: OpId = 2057;

pub type OpDispatcher = fn(
  op_id: OpId,
  state: &ThreadSafeState,
  control: &[u8],
  zero_copy: Option<PinnedBuf>,
) -> CoreOp;

pub struct DispatchManager {
  minimal: MinimalDispatcher,
  json: JsonDispatcher,
}

impl DispatchManager {
  pub fn new() -> Self {
    let minimal_dispatcher = MinimalDispatcher::new();
    minimal_dispatcher.register_op("read", io::op_read);
    minimal_dispatcher.register_op("write", io::op_write);

    let json_dispatcher = JsonDispatcher::new();
    json_dispatcher.register_op("exit", os::op_exit);
    json_dispatcher.register_op("is_tty", os::op_is_tty);
    json_dispatcher.register_op("env", os::op_env);
    json_dispatcher.register_op("exec_path", os::op_exec_path);
    json_dispatcher.register_op("home_dir", os::op_home_dir);
    json_dispatcher.register_op("utime", fs::op_utime);
    json_dispatcher.register_op("set_env", os::op_set_env);
    json_dispatcher.register_op("start", os::op_start);
    json_dispatcher
      .register_op("apply_source_map", errors::op_apply_source_map);
    json_dispatcher.register_op("format_error", errors::op_format_error);
    json_dispatcher.register_op("cache", compiler::op_cache);
    json_dispatcher
      .register_op("fetch_source_file", compiler::op_fetch_source_file);
    json_dispatcher.register_op("open", files::op_open);
    json_dispatcher.register_op("close", files::op_close);
    json_dispatcher.register_op("seek", files::op_seek);
    json_dispatcher.register_op("metrics", metrics::op_metrics);
    json_dispatcher.register_op("fetch", fetch::op_fetch);
    json_dispatcher.register_op("repl_start", repl::op_repl_start);
    json_dispatcher.register_op("repl_readline", repl::op_repl_readline);
    json_dispatcher.register_op("accept", net::op_accept);
    json_dispatcher.register_op("dial", net::op_dial);
    json_dispatcher.register_op("shutdown", net::op_shutdown);
    json_dispatcher.register_op("listen", net::op_listen);
    json_dispatcher.register_op("resources", resources::op_resources);
    json_dispatcher
      .register_op("get_random_values", random::op_get_random_values);
    json_dispatcher
      .register_op("global_timer_stop", timers::op_global_timer_stop);
    json_dispatcher.register_op("global_timer", timers::op_global_timer);
    json_dispatcher.register_op("now", performance::op_now);
    json_dispatcher.register_op("permissions", permissions::op_permissions);
    json_dispatcher
      .register_op("revoke_permission", permissions::op_revoke_permission);
    json_dispatcher.register_op("create_worker", workers::op_create_worker);
    json_dispatcher.register_op(
      "host_get_worker_closed",
      workers::op_host_get_worker_closed,
    );
    json_dispatcher
      .register_op("host_post_message", workers::op_host_post_message);
    json_dispatcher
      .register_op("host_get_message", workers::op_host_get_message);
    // TODO: make sure these two ops are only accessible to appropriate Worker
    json_dispatcher
      .register_op("worker_post_message", workers::op_worker_post_message);
    json_dispatcher
      .register_op("worker_get_message", workers::op_worker_get_message);
    json_dispatcher.register_op("run", process::op_run);
    json_dispatcher.register_op("run_status", process::op_run_status);
    json_dispatcher.register_op("kill", process::op_kill);
    json_dispatcher.register_op("chdir", fs::op_chdir);
    json_dispatcher.register_op("mkdir", fs::op_mkdir);
    json_dispatcher.register_op("chmod", fs::op_chmod);
    json_dispatcher.register_op("chown", fs::op_chown);
    json_dispatcher.register_op("remove", fs::op_remove);
    json_dispatcher.register_op("copy_file", fs::op_copy_file);
    json_dispatcher.register_op("stat", fs::op_stat);
    json_dispatcher.register_op("read_dir", fs::op_read_dir);
    json_dispatcher.register_op("rename", fs::op_rename);
    json_dispatcher.register_op("link", fs::op_link);
    json_dispatcher.register_op("symlink", fs::op_symlink);
    json_dispatcher.register_op("read_link", fs::op_read_link);
    json_dispatcher.register_op("truncate", fs::op_truncate);
    json_dispatcher.register_op("make_temp_dir", fs::op_make_temp_dir);
    json_dispatcher.register_op("cwd", fs::op_cwd);
    json_dispatcher.register_op("fetch_asset", compiler::op_fetch_asset);

    Self {
      minimal: minimal_dispatcher,
      json: json_dispatcher,
    }
  }

  pub fn register_minimal_op(
    &self,
    name: &str,
    handler: MinimalOpHandler,
  ) -> OpId {
    self.minimal.register_op(name, handler)
  }

  pub fn register_json_op(&self, name: &str, handler: JsonOpHandler) -> OpId {
    self.json.register_op(name, handler)
  }

  pub fn dispatch(
    &self,
    state: &ThreadSafeState,
    op_id: OpId,
    control: &[u8],
    zero_copy: Option<PinnedBuf>,
  ) -> CoreOp {
    let bytes_sent_control = control.len();
    let bytes_sent_zero_copy = zero_copy.as_ref().map(|b| b.len()).unwrap_or(0);

    let op = match op_id / 1000 {
      1 => self.minimal.dispatch(op_id, state, control, zero_copy),
      2 => self.json.dispatch(op_id, state, control, zero_copy),
      _ => panic!("unknown dispatch!"),
    };

    state.metrics_op_dispatched(bytes_sent_control, bytes_sent_zero_copy);

    match op {
      Op::Sync(buf) => {
        state.metrics_op_completed(buf.len());
        Op::Sync(buf)
      }
      Op::Async(fut) => {
        use crate::futures::Future;
        let state = state.clone();
        let result_fut = Box::new(fut.map(move |buf: Buf| {
          state.clone().metrics_op_completed(buf.len());
          buf
        }));
        Op::Async(result_fut)
      }
    }
  }
}
