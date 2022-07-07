use std::collections::HashMap;

pub struct WasiWorkloadRecord {
    pub name: String,
    pub language: String,
    pub size: String,
    pub opt_level: String,
    pub functions_executions: HashMap<String, u32>,
}

impl WasiWorkloadRecord {
    pub fn new() -> Self {
        WasiWorkloadRecord {
            name: String::default(),
            language: String::default(),
            size: String::default(),
            opt_level: String::default(),
            functions_executions: HashMap::new(),
        }
    }

    pub fn add_function_execution(&mut self, function_name: String) {
        if self.functions_executions.contains_key(&function_name) {
            *self.functions_executions.get_mut(&function_name).unwrap() += 1;
        } else {
            self.functions_executions.insert(function_name, 1);
        }
    }
}