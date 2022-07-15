use super::WasmFunctionBuilder;
use super::super::runtime::WasmExecutionState;
use wasmtime::{Linker, Store};
use anyhow::{Result};
use crate::workload::models::{Workload, ExecutionType};
use super::build_base_path;

pub struct FibonacciIterativeReactorFunctionBuilder;
impl WasmFunctionBuilder for FibonacciIterativeReactorFunctionBuilder {
    type InputType = i32;

    fn build_function(&self, workload: &Workload<i32>) -> Box<dyn Fn(&Linker<WasmExecutionState>, &mut Store<WasmExecutionState>) -> Result<()>> {
        let parameter = workload.size.value;

        let function = move |linker: &Linker<WasmExecutionState>, mut store: &mut Store<WasmExecutionState>| {
            let wasm_fun = linker.get(&mut store, "", "fibonacciiter").unwrap().into_func().unwrap().typed::<i32, i64, _>(&store)?;
            let result = wasm_fun.call(&mut store, parameter)?;

            store.data_mut().execution_result.set_successfull(validate_result(result, parameter));
            Ok(())
        };

        Box::new(function)
    }

    fn get_register_workload_name(&self) -> String {
        String::from("fibonacciiter")
    }

    fn get_register_type(&self) -> ExecutionType {
        ExecutionType::Reactor
    }
}

pub struct FibonacciIterativeCommanderFunctionBuilder;
impl WasmFunctionBuilder for FibonacciIterativeCommanderFunctionBuilder {
    type InputType = i32;

    fn build_function(&self, _workload: &Workload<i32>) -> Box<dyn Fn(&Linker<WasmExecutionState>, &mut Store<WasmExecutionState>) -> Result<()>> {
        let function = |linker: &Linker<WasmExecutionState>, mut store: &mut Store<WasmExecutionState>| {
            let wasm_fun = linker.get_default(&mut store, "")?.typed::<(), (), _>(&store)?;
            wasm_fun.call(&mut store, ())?;

            store.data_mut().execution_result.set_successfull(true);

            Ok(())
        };

        Box::new(function)
    }

    fn get_register_workload_name(&self) -> String {
        String::from("fibonacciiter")
    }

    fn get_register_type(&self) -> ExecutionType {
        ExecutionType::Commander
    }

    fn get_arguments(&self, workload: &Workload<i32>) -> Vec<String> {
        vec![String::from(workload.language.to_string()+ ".wasm"), String::from("fibonacciiter.rb"), workload.size.value.to_string()]
    }
    fn get_map_dirs(&self, workload: &Workload<i32>, base_dir: &str) -> Vec<(String, String)> {
        vec![(build_base_path(&workload.name, &workload.language, base_dir).into_os_string().into_string().unwrap(), String::from("/"))]
    }
}

fn validate_result(result: i64, input: i32) -> bool {
    let expected = match input {
        5 => 5,
        50 => 12586269025,
        90 => 2880067194370816120,
        _ => panic!("Error while validating fibonacci result. Unknown input."),
    };

    let success = expected == result;

    if !success {
        eprintln!("Fibonacci result was not correct. Expected {}, Acutal {}", expected, result);
    }

    success
}