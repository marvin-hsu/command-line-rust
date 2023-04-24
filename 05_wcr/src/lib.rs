use std::error::Error;

use clap::Command;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust wc")
        .get_matches();

    Ok(Config {
        files: Vec::new(),
        lines: false,
        words: false,
        bytes: false,
        chars: false,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}