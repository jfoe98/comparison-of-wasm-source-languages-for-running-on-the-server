use uuid::Uuid;

pub struct Workload<T> {
    pub name: String,
    pub id: Uuid,
    pub language: WorkloadLanguage,
    pub size: WorkloadSize<T>,
    pub opt_level: String,
    pub execution_type: ExecutionType,
}

#[derive(Clone, PartialEq)]
pub enum WorkloadLanguage {
    CEmscripten,
    CWasiSdk,
    Go,
    Ruby,
    Rust,
    Typescript,
}

#[derive(PartialEq)]
pub enum ExecutionType {
    Commander,
    Reactor
}

impl WorkloadLanguage {
    pub fn to_string(&self) -> String {
        match &self {
            WorkloadLanguage::CEmscripten => String::from("c-ems"),
            WorkloadLanguage::CWasiSdk => String::from("c-wasisdk"),
            WorkloadLanguage::Go => String::from("go"),
            WorkloadLanguage::Ruby => String::from("ruby"),
            WorkloadLanguage::Rust => String::from("rust"),
            WorkloadLanguage::Typescript => String::from("typescript"),
        }
    }

    pub fn get_opt_levels(&self) -> Vec<String> {
        match &self {
            WorkloadLanguage::Ruby => vec![String::from("default")],
            _ => vec![String::from("optimized"), String::from("unoptimized")],
        }
    }
}

impl ExecutionType {
    pub fn to_string(&self) -> String {
        match &self {
            ExecutionType::Commander => String::from("commander"),
            ExecutionType::Reactor => String::from("reactor"),
        }
    }
}

pub struct WorkloadSize<T> {
    pub size: Size,
    pub value: T,
}

impl<T: Copy> WorkloadSize<T> {
    pub fn new(size: Size, value: T) -> WorkloadSize<T> {
        WorkloadSize {
            size,
            value,
        }
    }
}

impl<T: Copy> Clone for WorkloadSize<T> {
    fn clone(&self) -> Self {
        WorkloadSize {
            size: self.size.clone(),
            value: self.value
        }
    }
}

#[derive(Clone)]
pub enum Size {
    Small,
    Medium,
    Large
}

impl Size {
    pub fn to_string(&self) -> String {
        match &self {
            Size::Small => String::from("small"),
            Size::Medium => String::from("medium"),
            Size::Large => String::from("large"),
        }
    }
}