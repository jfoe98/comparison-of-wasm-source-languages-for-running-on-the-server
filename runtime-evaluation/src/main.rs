use std::env;
use std::process;

use runtime_evaluation::config::{Config, Command};

#[tokio::main]
async fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    pretty_env_logger::try_init().unwrap();

    execute_command(config).await;
}

async fn execute_command(config: Config) {    
    match &config.command {
        Command::RunEvaluation => {
            runtime_evaluation::run_evaluation(&config).await.unwrap_or_else(|err| {
                eprintln!("Problem during evaluation: {}", err);
                process::exit(1);
            });
        },
        Command::ParseWasiLog(path) => {
            runtime_evaluation::parse_wasi_log(path.to_string(), &config).await.unwrap_or_else(|err| {
                eprintln!("Problem during parsing of wasi log: {}", err);
                process::exit(1);
            });
        }
    }
}