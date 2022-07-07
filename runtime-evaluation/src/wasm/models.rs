use std::path::PathBuf;
use std::time::{Instant, Duration};

use wasmtime::{Linker, Store};
use super::runtime::WasmExecutionState;
use anyhow::Result;

pub struct WasmRunCommand
{
    pub module: PathBuf,
    pub function: Box<dyn Fn(&Linker<WasmExecutionState>, &mut Store<WasmExecutionState>) -> Result<()>>,
    pub args: Vec<String>,
    pub map_dirs: Vec<(String, String)>
}

impl WasmRunCommand {
    pub fn new(module: PathBuf, function: impl Fn(&Linker<WasmExecutionState>, &mut Store<WasmExecutionState>) -> Result<()> + 'static, args: Vec<String>, map_dirs: Vec<(String, String)>) -> WasmRunCommand {
        WasmRunCommand {
            module,
            function: Box::new(function),
            args,
            map_dirs,
        }
    }
}

#[derive(Clone)]
pub struct ExecutionResult {
    instant: Option<Instant>,
    pub successfull: bool,
    pub startup_time: Option<Duration>,
    pub shutdown_time: Option<Duration>,
    pub execution_time: Option<Duration>,
    pub jit_compile_time: Option<Duration>,
    pub host_calls_count: u32,
}

impl ExecutionResult {
    pub fn new() -> ExecutionResult {
        ExecutionResult {
            instant: None,
            successfull: false,
            startup_time: None,
            shutdown_time: None,
            execution_time: None,
            jit_compile_time: None,
            host_calls_count: 0
        }
    }

    pub fn start_execution_measurement(&mut self) {
        self.instant = Some(Instant::now());
    }

    pub fn measure_startup(&mut self) {
        match self.instant {
            Some(instant) => self.startup_time = Some(instant.elapsed()),
            _ => panic!("Cannot measure startup when execution was not started before.")
        }
    }

    pub fn measure_shutdown(&mut self) {
        match self.instant {
            Some(instant) => self.shutdown_time = Some(instant.elapsed()),
            _ => panic!("Cannot measure startup when execution was not started before.")
        }
    }

    pub fn measure_execution(&mut self) {
        match self.instant {
            Some(instant) => self.execution_time = Some(instant.elapsed()),
            _ => panic!("Cannot measure startup when execution was not started before.")
        }
    }

    pub fn set_jit_compile_time(&mut self, time: Duration) {
        self.jit_compile_time = Some(time);
    }

    pub fn increase_host_call_count(&mut self) {
        self.host_calls_count += 1;
    }

    pub fn set_successfull(&mut self, value: bool) {
        self.successfull = value;
    }
}