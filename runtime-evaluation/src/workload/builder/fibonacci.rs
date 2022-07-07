use super::super::models::{WorkloadSize, Size};
use super::WorkloadBuilder;

pub struct FibonacciWorkloadBuilder;
impl WorkloadBuilder for FibonacciWorkloadBuilder {
    type WorkloadType = i32;

    fn get_name(&self) -> String { 
        String::from("fibonacci")
    }

    fn get_workload_sizes(&self) -> Vec<WorkloadSize<Self::WorkloadType>> { 
        vec![WorkloadSize::new(Size::Small, 5), WorkloadSize::new(Size::Medium, 30), WorkloadSize::new(Size::Large, 50)]
    }
}