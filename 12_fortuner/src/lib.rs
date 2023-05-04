use std::error::Error;

use clap::{Arg, ArgAction, Command};
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
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input files or directories")
                .action(ArgAction::Append)
                .num_args(1..),
        )
        .arg(
            Arg::new("insensitive")
                .help("Case-insensitive pattern matching")
                .action(ArgAction::SetTrue)
                .short('i')
                .long("insensitive"),
        )
        .arg(
            Arg::new("pattern")
                .help("Pattern")
                .short('m')
                .long("pattern")
                .value_name("PATTERN"),
        )
        .arg(
            Arg::new("seed")
                .help("Random seed")
                .short('s')
                .long("seed")
                .value_name("SEED"),
        )
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
