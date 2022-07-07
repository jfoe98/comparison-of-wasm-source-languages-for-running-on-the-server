mod runtime;
mod builder;
pub mod models;

use runtime::WasmtimeRuntime;
use models::ExecutionResult;
use anyhow::{Result};
use super::workload::models::Workload;
use builder::{WasmCommandBuilder, I32WasmCommandBuilder};

pub fn run_workload_i32(workload: &Workload<i32>, base_dir: &str) -> Result<ExecutionResult> {
    let runtime = WasmtimeRuntime::new();

    let wasm_command_builder = I32WasmCommandBuilder::new();
    let wasm_run_command = wasm_command_builder.build_wasm_command(workload, base_dir)?;

    runtime.run_wasm_binary(wasm_run_command)
}