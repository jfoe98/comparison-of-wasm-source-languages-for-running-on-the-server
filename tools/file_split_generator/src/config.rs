use std::env;

pub struct Config {
    pub numbers_count: i32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let numbers_count = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a numbers count"),
        };
    
        Ok(Config {numbers_count: numbers_count.parse::<i32>().expect("Error while parsing command line argument")})
    }
}