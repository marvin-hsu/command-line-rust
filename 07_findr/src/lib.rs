use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

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
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config.entry_types.iter().any(|t| match t {
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Link => entry.file_type().is_symlink(),
            })
    };

    let name_fiter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|n| n.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(e) => Some(e),
                Err(err) => {
                    eprintln!("{}", err);
                    None
                }
            })
            .filter(type_filter)
            .filter(name_fiter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<String>>();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}
