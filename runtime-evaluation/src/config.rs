use std::env;

pub struct Config {
    pub languages_base_path: String,
    pub metrics_base_uri: String,
    pub report_metrics: bool,
    pub repetitions: i8,
    pub command: Command,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let command = get_command(args.next(), args);
        
        let languages_base_path = match env::var("LANGUAGES_BASE_PATH"){
            Ok(arg) => arg,
            _ => String::from("../languages"),
        };

        let report_metrics = match env::var("NOT_REPORT_METRICS"){
            Ok(arg) => !(arg == "true"),
            _ => true,
        };

        let metrics_base_uri = match env::var("METRICS_BASE_URI"){
            Ok(arg) => arg,
            _ => String::from("http://10.95.9.113:3000"),
        };

        let repetitions = match env::var("REPETITIONS"){
            Ok(arg) => arg,
            _ => String::from("15"),
        };
    
        Ok(Config {languages_base_path, metrics_base_uri, repetitions: repetitions.parse::<i8>().unwrap(), command, report_metrics})
    }
}


fn get_command(command_arg: Option<String>, mut args: env::Args) -> Command {
    match command_arg {
        Some(cmd) => {
            if cmd == "parse-wasi-log" {
                let log_path = match args.next() {
                    Some(path) => path,
                    None => String::from("./wasi.log"),
                };

                return Command::ParseWasiLog(log_path)
            }
            Command::RunEvaluation
        },
        None => Command::RunEvaluation
    }
}

#[derive(PartialEq)]
pub enum Command {
    RunEvaluation,
    ParseWasiLog(String),
}
