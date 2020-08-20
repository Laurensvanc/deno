// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use super::dispatch_json::{Deserialize, JsonOp, Value};
use crate::diagnostics::Diagnostic;
use crate::errbox::from_serde;
use crate::source_maps::get_orig_position;
use crate::source_maps::CachedMaps;
use crate::state::State;
use deno_core::CoreIsolate;
use deno_core::ErrBox;
use deno_core::ZeroCopyBuf;
use std::collections::HashMap;
use std::rc::Rc;

pub fn init(i: &mut CoreIsolate, s: &Rc<State>) {
  i.register_op(
    "op_apply_source_map",
    s.stateful_json_op(op_apply_source_map),
  );
  i.register_op(
    "op_format_diagnostic",
    s.stateful_json_op(op_format_diagnostic),
  );
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApplySourceMap {
  file_name: String,
  line_number: i32,
  column_number: i32,
}

fn op_apply_source_map(
  state: &Rc<State>,
  args: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<JsonOp, ErrBox> {
  let args: ApplySourceMap =
    serde_json::from_value(args).map_err(from_serde)?;

  let mut mappings_map: CachedMaps = HashMap::new();
  let (orig_file_name, orig_line_number, orig_column_number) =
    get_orig_position(
      args.file_name,
      args.line_number.into(),
      args.column_number.into(),
      &mut mappings_map,
      &state.global_state.ts_compiler,
    );

  Ok(JsonOp::Sync(json!({
    "fileName": orig_file_name,
    "lineNumber": orig_line_number as u32,
    "columnNumber": orig_column_number as u32,
  })))
}

fn op_format_diagnostic(
  _state: &Rc<State>,
  args: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<JsonOp, ErrBox> {
  let diagnostic: Diagnostic =
    serde_json::from_value(args).map_err(from_serde)?;
  Ok(JsonOp::Sync(json!(diagnostic.to_string())))
}
