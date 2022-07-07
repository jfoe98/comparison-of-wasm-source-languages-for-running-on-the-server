use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead, prelude::*};
use std::fs::OpenOptions;
use rand::Rng;

use crate::config::Config;

mod config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut rng = rand::thread_rng();

    for _ in 0..config.numbers_count {
        write_to_file(rng.gen_range(1..100), config.numbers_count).unwrap();
    }
}

fn write_to_file(number: i32, number_count: i32) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(format!("./output/numbers_{}.txt", number_count))?;

    writeln!(file, "{}", number)
}