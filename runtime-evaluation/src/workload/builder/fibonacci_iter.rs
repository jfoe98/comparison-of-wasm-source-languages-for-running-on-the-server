use super::super::models::{WorkloadSize, Size};
use super::WorkloadBuilder;

pub struct FibonacciIterativeWorkloadBuilder;
impl WorkloadBuilder for FibonacciIterativeWorkloadBuilder {
    type WorkloadType = i32;

    fn get_name(&self) -> String { 
        String::from("fibonacciiter")
    }

    fn get_workload_sizes(&self) -> Vec<WorkloadSize<Self::WorkloadType>> { 
        vec![WorkloadSize::new(Size::Small, 5), WorkloadSize::new(Size::Medium, 50), WorkloadSize::new(Size::Large, 90)]
    }
}