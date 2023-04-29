use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

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

    let names = matches
        .get_many::<String>("name")
        .map(|vals| {
            vals.map(|val| Regex::new(val).map_err(|_| format!("Invalid --name \"{}\"", val)))
                .collect()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .get_many::<String>("type")
        .map(|vals| {
            vals.map(|val| match val.as_str() {
                "d" => EntryType::Dir,
                "f" => EntryType::File,
                "l" => EntryType::Link,
                _ => unreachable!("Invalid type!"),
            })
            .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths: matches
            .get_many::<String>("path")
            .unwrap()
            .cloned()
            .collect(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Ok(entry) => println!("{}", entry.path().display()),
                Err(e) => eprint!("{}", e),
            }
        }
    }
    Ok(())
}
