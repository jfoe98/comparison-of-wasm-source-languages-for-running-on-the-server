use super::WasmFunctionBuilder;
use super::super::runtime::WasmExecutionState;
use wasmtime::{Linker, Store};
use anyhow::{Result};
use crate::workload::models::{Workload, ExecutionType, WorkloadLanguage};
use super::build_base_path;
use std::fs;
use uuid::Uuid;
use std::fs::File;
use std::io::{self, BufRead};
use std::fs::OpenOptions;
use std::path::Path;

pub struct FileSplitReactorFunctionBuilder;
impl WasmFunctionBuilder for FileSplitReactorFunctionBuilder {
    type InputType = i32;

    fn build_function(&self, workload: &Workload<i32>) -> Box<dyn Fn(&Linker<WasmExecutionState>, &mut Store<WasmExecutionState>) -> Result<()>> {
        let parameter = workload.size.value;
        let workload_id = workload.id;
        let workload_size = workload.size.size.to_string();
        let language = workload.language.to_string();

        let function = move |linker: &Linker<WasmExecutionState>, mut store: &mut Store<WasmExecutionState>| {
            let wasm_fun = linker.get(&mut store, "", "filesplit").unwrap().into_func().unwrap().typed::<i32, (), _>(&store)?;
            wasm_fun.call(&mut store, parameter)?;

            store.data_mut().execution_result.set_successfull(validate_result(parameter, &workload_size, &language, workload_id));
            Ok(())
        };

        Box::new(function)
    }

    fn get_register_workload_name(&self) -> String {
        String::from("filesplit")
    }

    fn get_register_type(&self) -> ExecutionType {
        ExecutionType::Reactor
    }

    fn get_map_dirs(&self, workload: &Workload<i32>, _base_dir: &str) -> Vec<(String, String)> {
        vec![
            get_output_map_dir(workload, None)
        ]
    }
}

pub struct FileSplitCommanderFunctionBuilder;
impl WasmFunctionBuilder for FileSplitCommanderFunctionBuilder {
    type InputType = i32;

    fn build_function(&self, workload: &Workload<i32>) -> Box<dyn Fn(&Linker<WasmExecutionState>, &mut Store<WasmExecutionState>) -> Result<()>> {
        let parameter = workload.size.value;
        let workload_id = workload.id;
        let workload_size = workload.size.size.to_string();
        let language = workload.language.to_string();
        
        let function = move |linker: &Linker<WasmExecutionState>, mut store: &mut Store<WasmExecutionState>| {
            let wasm_fun = linker.get_default(&mut store, "")?.typed::<(), (), _>(&store)?;
            wasm_fun.call(&mut store, ())?;

            store.data_mut().execution_result.set_successfull(validate_result(parameter, &workload_size, &language, workload_id));
            Ok(())
        };

        Box::new(function)
    }

    fn get_register_workload_name(&self) -> String {
        String::from("filesplit")
    }

    fn get_register_type(&self) -> ExecutionType {
        ExecutionType::Commander
    }

    fn get_arguments(&self, workload: &Workload<i32>) -> Vec<String> {
        vec![String::from(workload.language.to_string()+ ".wasm"), String::from("filesplit.rb"), workload.size.value.to_string()]
    }

    fn get_map_dirs(&self, workload: &Workload<i32>, base_dir: &str) -> Vec<(String, String)> {
        let mut map_dirs = Vec::new();

        map_dirs.push((build_base_path(&workload.name, &workload.language, base_dir).into_os_string().into_string().unwrap(), String::from("/")));

        if workload.language == WorkloadLanguage::Ruby {
            map_dirs.push(get_output_map_dir(workload, Some("./workload")));
        } else {
            map_dirs.push(get_output_map_dir(workload, None));
        }

        map_dirs
    }
}

fn validate_result(input: i32, size: &str, language: &str, workload_id: Uuid) -> bool {
    let mut counter = 0;

    for i in 0..10 {
        let path = get_output_file(i, language, size, workload_id);

        if !Path::new(&path).exists() {
            continue;
        }

        if let Ok(file) = get_file(&path) {
            let lines = read_lines(file);
    
            for line in lines {
                if let Ok(ip) = line {
                    if ip.is_empty() {
                        continue;
                    }

                    let number = ip.trim().parse::<i32>().unwrap();

                    counter = counter + 1;
                    
                    if number % 10 != i {
                        eprintln!("Wrong number {} found in file {}", number, &path);
                        return false;
                    }
                }
            }
        }
    }

    if counter != input {
        eprintln!("Not all numbers have been splitted into text files. Expected: {}, Actual {}", input, counter);

        return false;
    }

    true
}

fn get_output_map_dir(workload: &Workload<i32>, dir: Option<&str>) -> (String, String) {
    let workload_dir = get_output_path(&workload.language.to_string(), &workload.size.size.to_string(), workload.id);
    fs::create_dir_all(&workload_dir).unwrap();
    fs::copy(format!("./files/input/numbers_{}.txt", workload.size.value), format!("{}/numbers_{}.txt", workload_dir, workload.size.value)).unwrap();

    match dir {
        Some(directory) => (String::from(workload_dir), String::from(directory)),
        None =>  (String::from(workload_dir), String::from(".")),
    }
}

fn get_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .read(true)
        .open(path)
}

fn read_lines(file: File) -> io::Lines<io::BufReader<File>> {
    io::BufReader::new(file).lines()
}

fn get_output_path(language: &str, size: &str, id: Uuid) -> String {
    format!("./output/{}/{}/{}", language, size, id)
}

fn get_output_file(number: i32, language: &str, size: &str, id: Uuid) -> String {
    format!("{}/{}.txt", get_output_path(language, size, id), number)
}