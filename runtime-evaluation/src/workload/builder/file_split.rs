use super::super::models::{WorkloadLanguage, WorkloadSize, Size, ExecutionType};
use super::WorkloadBuilder;

pub struct FileSplitWorkloadBuilder;
impl WorkloadBuilder for FileSplitWorkloadBuilder {
    type WorkloadType = i32;

    fn get_name(&self) -> String { 
        String::from("filesplit")
    }

    fn get_workload_sizes(&self) -> Vec<WorkloadSize<Self::WorkloadType>> { 
        vec![WorkloadSize::new(Size::Small, 1000), WorkloadSize::new(Size::Medium, 10000), WorkloadSize::new(Size::Large, 100000)]
    }

    fn get_execution_type(&self, language: &WorkloadLanguage) -> ExecutionType {
        // override default implementation because Go needed to be implemented as commander
        match language {
            WorkloadLanguage::CEmscripten | WorkloadLanguage::Rust | WorkloadLanguage::Typescript => ExecutionType::Reactor,
            WorkloadLanguage::Ruby | WorkloadLanguage::Go | WorkloadLanguage::CWasiSdk => ExecutionType::Commander,
        }
    }
}