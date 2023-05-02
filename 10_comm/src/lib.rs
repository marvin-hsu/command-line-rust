use std::error::Error;

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("commr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust comm")
        .arg(
            Arg::new("file1")
                .value_name("FILE1")
                .required(true)
                .num_args(1)
                .help("Input file 1"),
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .required(true)
                .num_args(1)
                .help("Input file 2"),
        )
        .arg(
            Arg::new("suppress_col1")
                .short('1')
                .help("Suppress printing of column 1")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("suppress_col2")
                .short('2')
                .help("Suppress printing of column 2")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("suppress_col3")
                .short('3')
                .help("Suppress printing of column 3")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("insentive")
                .short('i')
                .help("Case-insensitive comparsion of lines"),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .default_value("\t"),
        )
        .get_matches();
    Ok(Config {})
}

pub fn run(config: Config) -> MyResult<()> {
    todo!();
    Ok(())
}
