mod models;
mod parser;

use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::fs::OpenOptions;
use models::WasiWorkloadRecord;
use parser::LogStatement;
use crate::metrics::models::Metric;
use crate::config::Config;

pub async fn parse_wasi_log(log_path: String, config: &Config) -> Result<()> {
    let file = get_file(&log_path)?;
    let lines = read_lines(file);

    let mut workload_record = WasiWorkloadRecord::new();

    for line in lines {
        if let Ok(log) = line {
            if log.is_empty() {
                continue;
            }

            let log_statement = parser::parse_line(&log);

            match log_statement {
                LogStatement::NewWorkload => {
                    if !workload_record.name.is_empty() {
                        write_metrics(workload_record, &config).await?;
                    }

                    workload_record = WasiWorkloadRecord::new();
                },
                LogStatement::WorkloadName(name) => workload_record.name = name,
                LogStatement::WorkloadLanguage(language) => workload_record.language = language,
                LogStatement::WorkloadSize(size) => workload_record.size = size,
                LogStatement::WorkloadOptLevel(opt_level) => workload_record.opt_level = opt_level,
                LogStatement::WasiFunctionCall(function_call) => workload_record.add_function_execution(function_call),
                LogStatement::Unknown => (),
            }
        }
    }

    Ok(())
}

async fn write_metrics(record: WasiWorkloadRecord, config: &Config) -> Result<()> {
    let context = crate::metrics::build_context(&record.name, &record.opt_level, &record.size);

    for function_call in record.functions_executions {
        let metric = Metric::new(record.language.clone(), format!("wasi-{}", function_call.0), context.clone(), function_call.1.to_string());
        crate::metrics::write_metric(config, &metric).await?;
    }

    Ok(())
}

fn get_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .read(true)
        .open(path)
}

fn read_lines(file: File) -> io::Lines<io::BufReader<File>> {
    io::BufReader::new(file).lines()
}