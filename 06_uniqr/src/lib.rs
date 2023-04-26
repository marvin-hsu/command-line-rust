use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_files: String,
    out_files: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust uniq")
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Show counts")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
                .num_args(0..=1),
        )
        .get_matches();

    Ok(Config {
        in_files: matches.get_one::<String>("in_file").unwrap().clone(),
        out_files: matches.get_one::<String>("out_file").cloned(),
        count: matches.get_flag("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}
