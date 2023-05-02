use std::error::Error;

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

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
                .help("Case-insensitive comparsion of lines")
                .action(ArgAction::SetTrue),
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
    Ok(Config {
        file1: matches.get_one::<String>("file1").unwrap().clone(),
        file2: matches.get_one::<String>("file2").unwrap().clone(),
        show_col1: matches.get_flag("suppress_col1"),
        show_col2: matches.get_flag("suppress_col2"),
        show_col3: matches.get_flag("suppress_col3"),
        insensitive: matches.get_flag("insentive"),
        delimiter: matches.get_one::<String>("delimiter").unwrap().clone(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}",config);
    Ok(())
}
