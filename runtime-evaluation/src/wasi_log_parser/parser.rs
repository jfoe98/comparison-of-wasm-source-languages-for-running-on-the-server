use fancy_regex::Regex;

pub enum LogStatement {
    NewWorkload,
    WorkloadName(String),
    WorkloadLanguage(String),
    WorkloadSize(String),
    WorkloadOptLevel(String),
    WasiFunctionCall(String),
    Unknown,
}

pub fn parse_line(line: &str) -> LogStatement {
    if line.is_empty() {
        return LogStatement::Unknown;
    }

    if line.contains("Start of workload execution") {
        println!("[Parser] Start of new workload detected");
        return LogStatement::NewWorkload;
    }

    if let Some(workload_name) = regex_match(r#"(?<=Name:\s)\S{1,}"#, line) {
        println!("[Parser] Workload name: {}", workload_name);
        return LogStatement::WorkloadName(workload_name.clone());
    }

    if let Some(workload_language) = regex_match(r#"(?<=Language:\s)\S{1,}"#, line) {
        println!("[Parser] Workload language: {}", workload_language);
        return LogStatement::WorkloadLanguage(workload_language.clone());
    }

    if let Some(workload_size) = regex_match(r#"(?<=Size:\s)\S{1,}"#, line) {
        println!("[Parser] Workload size: {}", workload_size);
        return LogStatement::WorkloadSize(workload_size.clone());
    }

    if let Some(workload_opt_level) = regex_match(r#"(?<=Opt-Level:\s)\S{1,}"#, line) {
        println!("[Parser] Workload opt level: {}", workload_opt_level);
        return LogStatement::WorkloadOptLevel(workload_opt_level.clone());
    }

    if line.contains(r#"TRACE wasi_common::snapshots::preview_1::wasi_snapshot_preview1 > wiggle abi; module="wasi_snapshot_preview1""#) {
        if let Some(wasi_function_call) = regex_match(r#"(?<=function=")[^\s"]*"#, line) {
            println!("[Parser] Wasi function call: {}", wasi_function_call);
            return LogStatement::WasiFunctionCall(wasi_function_call.clone());
        }
    }

    LogStatement::Unknown
}

fn regex_match(regex: &str, line: &str) -> Option<String> {
    let regex = Regex::new(regex).unwrap();
    
    let regex_match = regex.find(line).unwrap_or_else(|err| {
        println!("Error while executing regex: {}", err);
        return None;
    });

    let result = match regex_match {
        Some(m) => Some(m.as_str().to_string()),
        None => None,
    };

    result
}