use std::path::{Path, PathBuf};
use super::runtime::WasmExecutionState;
use super::models::WasmRunCommand;
use wasmtime::{Linker, Store};
use anyhow::{Result, bail};
use super::super::workload::models::{Workload, WorkloadLanguage, ExecutionType};
use fibonacci_iter::{FibonacciIterativeReactorFunctionBuilder, FibonacciIterativeCommanderFunctionBuilder};
use fibonacci::{FibonacciReactorFunctionBuilder, FibonacciCommanderFunctionBuilder};
use file_split::{FileSplitReactorFunctionBuilder, FileSplitCommanderFunctionBuilder};

mod fibonacci_iter;
mod fibonacci;
mod file_split;

pub trait WasmCommandBuilder {
    type InputType;

    fn build_wasm_command(&self, workload: &Workload<Self::InputType>, base_dir: &str) -> Result<WasmRunCommand>;
}

pub struct I32WasmCommandBuilder {
    function_builder_factory: WasmFunctionBuilderFactory,
}

impl I32WasmCommandBuilder {
    pub fn new() -> Self {
        I32WasmCommandBuilder {
            function_builder_factory: WasmFunctionBuilderFactory::new()
        }
    }
}

impl WasmCommandBuilder for I32WasmCommandBuilder {
    type InputType = i32;

    fn build_wasm_command(&self, workload: &Workload<i32>, base_dir: &str) -> Result<WasmRunCommand> { 
        let function_builder = self.function_builder_factory.get_function_builder_i32(workload)?;

        let function = function_builder.build_function(workload);
        let arguments = function_builder.get_arguments(workload);
        let map_dirs = function_builder.get_map_dirs(workload, base_dir);

        Ok(WasmRunCommand::new(function_builder.get_module(workload, base_dir), function, arguments, map_dirs))
    }
}

trait WasmFunctionBuilder {
    type InputType;

    fn build_function(&self, workload: &Workload<Self::InputType>) -> Box<dyn Fn(&Linker<WasmExecutionState>, &mut Store<WasmExecutionState>) -> Result<()>>;
    fn get_register_workload_name(&self) -> String;
    fn get_register_type(&self) -> ExecutionType;
    fn get_arguments(&self, _workload: &Workload<Self::InputType>) -> Vec<String> {
        Vec::new()
    }
    fn get_map_dirs(&self, _workload: &Workload<Self::InputType>, _base_dir: &str) -> Vec<(String, String)> {
        Vec::new()
    }
    fn get_module(&self, workload: &Workload<Self::InputType>, base_dir: &str) -> PathBuf {
        match self.get_register_type() {
            ExecutionType::Reactor => build_standalone_binary_path(&workload.name, &workload.language, &workload.opt_level, base_dir),
            ExecutionType::Commander => {
                if workload.language == WorkloadLanguage::Ruby {
                    return build_interpreter_binary_path(&workload.language, base_dir);
                }

                build_standalone_binary_path(&workload.name, &workload.language, &workload.opt_level, base_dir)
            },
        }
    }
}

struct WasmFunctionBuilderFactory;
impl WasmFunctionBuilderFactory {
    pub fn new() -> Self {
        WasmFunctionBuilderFactory {}
    }

    fn get_function_builder_i32(&self, workload: &Workload<i32>) -> Result<Box<dyn WasmFunctionBuilder<InputType = i32>>> {
        let function_builders = self.get_function_builders_i32();

        for function_builder in function_builders {
            let builder = function_builder.as_ref();

            if builder.get_register_type() == workload.execution_type && builder.get_register_workload_name() == workload.name {
                return Ok(function_builder);
            }
        }

        bail!("No function builder found for name {} and type {}", workload.name, workload.execution_type.to_string());
    }

    fn get_function_builders_i32(&self) -> Vec<Box<dyn WasmFunctionBuilder<InputType = i32>>> {
        vec![
            Box::new(FibonacciIterativeReactorFunctionBuilder{}),
            Box::new(FibonacciIterativeCommanderFunctionBuilder{}),
            Box::new(FibonacciReactorFunctionBuilder{}),
            Box::new(FibonacciCommanderFunctionBuilder{}),
            Box::new(FileSplitReactorFunctionBuilder{}),
            Box::new(FileSplitCommanderFunctionBuilder{}),
        ]
    }
}

fn build_interpreter_binary_path(language: &WorkloadLanguage, base_dir: &str) -> PathBuf {
    Path::new(base_dir)
        .join(get_language_path(language))
        .join(get_language_path(language) + ".wasm")
}

fn build_standalone_binary_path(name: &str, language: &WorkloadLanguage, opt_level: &str, base_dir: &str) -> PathBuf {
    build_base_path(name, language, base_dir)
        .join("output")
        .join(String::from(name) + "_" + opt_level + ".wasm")
}

fn build_base_path(name: &str, language: &WorkloadLanguage, base_dir: &str) -> PathBuf {
    Path::new(base_dir)
        .join(get_language_path(language))
        .join(name)
}

fn get_language_path(language: &WorkloadLanguage) -> String {
    language.to_string()
}