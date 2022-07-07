pub mod config;
mod metrics;
mod wasm;
mod workload;
mod wasi_log_parser;

use config::Config;
use metrics::models::Metric;
use wasm::models::ExecutionResult;
use workload::{models::Workload, builder::WorkloadBuilderFactory};
use anyhow::Result;
use rand::thread_rng;
use rand::seq::SliceRandom;
use colored::Colorize;
use std::time::Duration;

pub async fn run_evaluation(config: &Config) -> Result<()> {
    let workloads = build_workloads_i32(config);
    let workload_count = workloads.len();

    for (index, workload) in workloads.iter().enumerate() {
        log_start(workload, index + 1, workload_count);

        let execution_result = wasm::run_workload_i32(&workload, &config.languages_base_path).unwrap_or_else(|error| {
            log_error(error);
            ExecutionResult::new()
        });

        write_metrics(config, &workload, &execution_result).await.unwrap_or_else(|error| {
            log_error(error);
        });

        log_end(&execution_result);
    }

    Ok(())
}

pub async fn parse_wasi_log(log_path: String, config: &Config) -> Result<()> {
    wasi_log_parser::parse_wasi_log(log_path, config).await?;

    Ok(())
}

fn build_workloads_i32(config: &Config) -> Vec<Workload<i32>> {
    let mut workloads: Vec<Workload<i32>> = Vec::new();
    let workload_builder_factory = WorkloadBuilderFactory::new();

    for workload_builder in workload_builder_factory.get_workload_builders_i32() {
        let new_workloads = workload_builder.as_ref().build_workloads(config.repetitions);
        for workload in new_workloads {
            workloads.push(workload);
        }
    }
    
    workloads.shuffle(&mut thread_rng());
    workloads
}

async fn write_metrics<'a, T>(config: &Config, workload: &Workload<T>, execution_result: &ExecutionResult) -> Result<()> {
    if !config.report_metrics {
        return Ok(());
    }

    write_metric(config, &workload, String::from("execution_time_micro"), unwrap_duration_micro_seconds(execution_result.execution_time)).await?;
    write_metric(config, &workload, String::from("execution_time_nano"), unwrap_duration_nano_seconds(execution_result.execution_time)).await?;
    write_metric(config, &workload, String::from("startup_time_micro"), unwrap_duration_micro_seconds(execution_result.startup_time)).await?;
    write_metric(config, &workload, String::from("startup_time_nano"), unwrap_duration_nano_seconds(execution_result.startup_time)).await?;

    let shutdown_time = unwrap_duration_micro_seconds(execution_result.shutdown_time);
    if shutdown_time > 0 {
        write_metric(config, &workload, String::from("shutdown_time_micro"), unwrap_duration_micro_seconds(execution_result.execution_time) - shutdown_time).await?;
        write_metric(config, &workload, String::from("shutdown_time_nano"), unwrap_duration_nano_seconds(execution_result.execution_time) - unwrap_duration_nano_seconds(execution_result.shutdown_time)).await?;
    }

    write_metric(config, &workload, String::from("jit_compile_time"), unwrap_duration_micro_seconds(execution_result.jit_compile_time)).await?;
    write_metric(config, &workload, String::from("host_calls_count"), execution_result.host_calls_count.into()).await?;

    let success = if execution_result.successfull {
        1
    } else {
        0
    };

    write_metric(config, &workload, String::from("success"), success).await?;

    Ok(())
}

async fn write_metric<T>(config: &Config, workload: &Workload<T>, metric: String, value: u128) -> Result<()> {
    let context = metrics::build_context(&workload.name, &workload.opt_level, &workload.size.size.to_string());

    let metric = Metric::new(workload.language.to_string(), metric, context, value.to_string());
    metrics::write_metric(config, &metric).await?;

    Ok(())
}

fn unwrap_duration_micro_seconds(duration: Option<Duration>) -> u128 {
    match duration {
        Some(dur) => dur.as_micros(),
        None => 0,
    }
}

fn unwrap_duration_nano_seconds(duration: Option<Duration>) -> u128 {
    match duration {
        Some(dur) => dur.as_nanos(),
        None => 0,
    }
}

fn log_start<T>(workload: &Workload<T>, number: usize, total_number: usize) {
    let start_message = format!("Start of workload execution ({} of {})", number, total_number);

    println!("{}", build_status_line(&start_message).blue());
    println!("Id: {}", workload.id);
    println!("Name: {}", workload.name);
    println!("Language: {}", workload.language.to_string());
    println!("Factors:");
    println!("  Size: {}", workload.size.size.to_string());
    println!("  Opt-Level: {}", workload.opt_level);
}

fn log_end(execution_result: &ExecutionResult) {
    println!("{}", build_status_line("Execution finished").blue());
    println!("JIT compile time: {}", unwrap_duration_micro_seconds(execution_result.jit_compile_time));
    println!("Startup time: {}", unwrap_duration_micro_seconds(execution_result.startup_time));
    println!("Shutdown time: {}", unwrap_duration_micro_seconds(execution_result.execution_time) - unwrap_duration_micro_seconds(execution_result.shutdown_time));
    println!("Execution time: {}", unwrap_duration_micro_seconds(execution_result.execution_time));
    println!("Host calls count : {}", execution_result.host_calls_count);

    let success = if execution_result.successfull {
        execution_result.successfull.to_string().green()
    } else {
        execution_result.successfull.to_string().red()
    };

    println!("Sucessfull: {}", success);
    println!("{}", build_status_line("End of workload execution").blue());
    println!();
}

fn log_error(error: anyhow::Error) {
    let error_message = format!("An error occured: {:#}", error);
    eprintln!("{}", error_message.red());
}

fn build_status_line(text: &str) -> String {
    let status_line_length = 80;

    if text.len() >= status_line_length {
        return String::from(text);
    }

    let diff = status_line_length - text.len();

    let left_side = diff / 2;
    let right_side = diff - left_side;

    let mut status_line = String::new();

    for _ in 0..left_side {
        status_line.push_str("#")
    }

    status_line.push_str(" ");
    status_line.push_str(text);
    status_line.push_str(" ");

    for _ in 0..right_side {
        status_line.push_str("#")
    }

    status_line
}