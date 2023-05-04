use std::error::Error;

use clap::Command;
use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    source: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("fortuner")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust fortune")
        .get_matches();

    Ok(Config {
        source: todo!(),
        pattern: todo!(),
        seed: todo!(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
