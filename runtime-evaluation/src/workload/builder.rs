use super::models::{WorkloadSize, Workload, WorkloadLanguage, ExecutionType};
use uuid::Uuid;
use fibonacci_iter::FibonacciIterativeWorkloadBuilder;
use fibonacci::FibonacciWorkloadBuilder;
use file_split::FileSplitWorkloadBuilder;

mod fibonacci;
mod fibonacci_iter;
mod file_split;

pub trait WorkloadBuilder {
    type WorkloadType: Copy;

    fn get_name(&self) -> String;
    fn get_workload_sizes(&self) -> Vec<WorkloadSize<Self::WorkloadType>>;

    fn build_workloads(&self, repetitions: i8) -> Vec<Workload<Self::WorkloadType>> {
        let mut workloads: Vec<Workload<Self::WorkloadType>> = Vec::new();
        let workload_sizes = self.get_workload_sizes();
        let languages = self.get_languages();

        for language in &languages {
            let opt_levels = language.get_opt_levels();

            for workload_size in &workload_sizes {
                for opt_level in &opt_levels {
                    for _ in 0..repetitions {
                        let workload = Workload {
                            id: Uuid::new_v4(),
                            name: self.get_name(),
                            language: language.clone(),
                            size: workload_size.clone(),
                            opt_level: opt_level.clone(),
                            execution_type: self.get_execution_type(&language)
                        };
                        
                        workloads.push(workload);
                    }
                }
            }
        }

        workloads
    }

    fn get_languages(&self) -> Vec<WorkloadLanguage> {
        vec![WorkloadLanguage::CEmscripten, WorkloadLanguage::CWasiSdk, WorkloadLanguage::Go, WorkloadLanguage::Rust, WorkloadLanguage::Typescript, WorkloadLanguage::Ruby]
    }

    fn get_execution_type(&self, language: &WorkloadLanguage) -> ExecutionType {
        match language {
            WorkloadLanguage::CEmscripten | WorkloadLanguage::Go | WorkloadLanguage::Rust | WorkloadLanguage::Typescript => ExecutionType::Reactor,
            WorkloadLanguage::Ruby | WorkloadLanguage::CWasiSdk => ExecutionType::Commander,
        }
    }
}

pub struct WorkloadBuilderFactory;
impl WorkloadBuilderFactory {
    pub fn new() -> Self {
        WorkloadBuilderFactory {}
    }

    pub fn get_workload_builders_i32(&self) -> Vec<Box<dyn WorkloadBuilder<WorkloadType = i32>>> {
        vec![Box::new(FibonacciIterativeWorkloadBuilder{}), Box::new(FibonacciWorkloadBuilder{}), Box::new(FileSplitWorkloadBuilder{})]
    }
}