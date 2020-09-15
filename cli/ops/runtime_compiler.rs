// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

use crate::tsc::runtime_bundle;
use crate::tsc::runtime_compile;
use crate::tsc::runtime_transpile;
use deno_core::error::AnyError;
use deno_core::BufVec;
use deno_core::OpState;
use serde_derive::Deserialize;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn init(rt: &mut deno_core::JsRuntime) {
  super::reg_json_async(rt, "op_compile", op_compile);
  super::reg_json_async(rt, "op_transpile", op_transpile);
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CompileArgs {
  root_name: String,
  sources: Option<HashMap<String, String>>,
  bundle: bool,
  options: Option<String>,
}

async fn op_compile(
  state: Rc<RefCell<OpState>>,
  args: Value,
  _data: BufVec,
) -> Result<Value, AnyError> {
  let cli_state = super::cli_state2(&state);
  cli_state.check_unstable("Deno.compile");
  let args: CompileArgs = serde_json::from_value(args)?;
  let global_state = cli_state.global_state.clone();
  let permissions = cli_state.permissions.borrow().clone();
  let response = if args.bundle {
    let r = runtime_bundle(
      &global_state,
      permissions,
      &args.root_name,
      &args.sources,
      &args.options,
    )
    .await?;
    serde_json::to_value(r)?
  } else {
    let r = runtime_compile(
      &global_state,
      permissions,
      &args.root_name,
      &args.sources,
      &args.options,
    )
    .await?;
    serde_json::to_value(r)?
  };

  Ok(response)
}

#[derive(Deserialize, Debug)]
struct TranspileArgs {
  sources: HashMap<String, String>,
  options: Option<String>,
}

async fn op_transpile(
  state: Rc<RefCell<OpState>>,
  args: Value,
  _data: BufVec,
) -> Result<Value, AnyError> {
  let cli_state = super::cli_state2(&state);
  cli_state.check_unstable("Deno.transpile");
  let args: TranspileArgs = serde_json::from_value(args)?;
  let global_state = cli_state.global_state.clone();
  let permissions = cli_state.permissions.borrow().clone();
  let result =
    runtime_transpile(&global_state, permissions, &args.sources, &args.options)
      .await?;
  Ok(result)
}
