use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust grep")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true),
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(0..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Count occurrences")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("invert-match")
                .short('v')
                .long("invert-match")
                .help("Invert match")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("Recursive search")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {})
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
