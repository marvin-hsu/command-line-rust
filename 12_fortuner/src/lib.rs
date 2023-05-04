use std::error::Error;

use clap::{Arg, ArgAction, Command};
use regex::{Regex, RegexBuilder};

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
                .required(true)
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

    let pattern = matches
        .get_one::<String>("pattern")
        .map(|s| {
            RegexBuilder::new(s)
                .case_insensitive(matches.get_flag("insensitive"))
                .build()
                .map_err(|_| format!("Invalid --pattern \"{}\"", s))
        })
        .transpose()?;

    let seed = matches
        .get_one::<String>("seed")
        .map(|s| {
            s.parse::<u64>()
                .map_err(|_| format!("\"{}\" not a valid integer", s))
        })
        .transpose()?;

    Ok(Config {
        source: matches
            .get_many::<String>("file")
            .unwrap()
            .cloned()
            .collect(),
        pattern,
        seed,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
