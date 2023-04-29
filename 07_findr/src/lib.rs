use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust find")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Name")
                .num_args(0..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Entry type")
                .value_parser(["f", "d", "l"])
                .num_args(0..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .num_args(0..)
                .action(ArgAction::Append),
        )
        .get_matches();

    let names = matches.get_many::<String>("name").map(|vals| {
        vals.map(|name| Regex::new(name).map_err(|_| format!("Invalid --name \"{}\"", name)))
            .collect::<Result<Vec<_>, _>>();
    });

    Ok(Config {
        paths: Vec::new(),
        names: Vec::new(),
        entry_types: Vec::new(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
