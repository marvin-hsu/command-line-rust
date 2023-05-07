use std::error::Error;

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("lsr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust ls")
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Show all files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Long listing")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Files and/or directories")
                .default_value(".")
                .num_args(0..)
                .action(ArgAction::Append),
        )
        .get_matches();

    Ok(Config {
        paths: matches
            .get_many::<String>("paths")
            .unwrap()
            .cloned()
            .collect(),
        long: matches.get_flag("long"),
        show_hidden: matches.get_flag("all"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
