
use wasmtime::*;
use wasmtime_wasi::{WasiCtx, sync::WasiCtxBuilder};
use wasmtime_wasi::sync::{ambient_authority, Dir};
use anyhow::Result;
use std::time::{Instant};
use super::models::{WasmRunCommand, ExecutionResult};

pub struct WasmExecutionState {
    pub execution_result: ExecutionResult,
    wasi: WasiCtx,
}

pub struct WasmtimeRuntime {
    engine: Engine,
}

impl WasmtimeRuntime {
    pub fn new() -> WasmtimeRuntime {
        let engine = Engine::default();

        WasmtimeRuntime {engine}
    }

    pub fn run_wasm_binary(&self, wasm_run_command: WasmRunCommand) -> Result<ExecutionResult> 
    {
        let engine = &self.engine;
        let mut execution_result = ExecutionResult::new();

        let jit_time_tracker = Instant::now();
        let module = Module::from_file(&engine, wasm_run_command.module)?;
        execution_result.set_jit_compile_time(jit_time_tracker.elapsed());
    
        let mut linker = Linker::new(&engine);

        wasmtime_wasi::add_to_linker(&mut linker, |state: &mut WasmExecutionState| &mut state.wasi)?;

        let mut wasi_context_builder = WasiCtxBuilder::new();

        for (host, guest) in wasm_run_command.map_dirs {
            wasi_context_builder = wasi_context_builder.preopened_dir(Dir::open_ambient_dir(host, ambient_authority())?, guest)?;
        }

        let wasi = wasi_context_builder
            .args(&wasm_run_command.args)?
            .inherit_env()?
            .inherit_stdin().inherit_stdout().inherit_stderr()
            .build();

        linker.func_wrap("env", "startup", |mut caller: Caller<'_, WasmExecutionState>| {
            caller.data_mut().execution_result.measure_startup();
        })?;

        linker.func_wrap("env", "finish", |mut caller: Caller<'_, WasmExecutionState>| {
            caller.data_mut().execution_result.measure_shutdown();
        })?;
        
        let mut store = Store::new(
            &engine,
            WasmExecutionState {
                execution_result,
                wasi,
            }
        );

        store.call_hook(|store, hook: CallHook| {
            match hook {
                CallHook::CallingWasm => store.execution_result.start_execution_measurement(),
                CallHook::ReturningFromWasm => store.execution_result.measure_execution(),
                CallHook::CallingHost => store.execution_result.increase_host_call_count(),
                _ => (),
            }

            return Ok(());
        });

        linker.module(&mut store, "", &module)?;

        (wasm_run_command.function)(&linker, &mut store)?;

        Ok(store.data().execution_result.clone())
    }
}