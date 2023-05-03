use std::error::Error;

use clap::Command;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("tailr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust tail")
        .get_matches();

    Ok(Config {})
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
