mod owner;

use std::{error::Error, fs, os::unix::prelude::MetadataExt, path::PathBuf};

use chrono::{DateTime, Local};
use clap::{Arg, ArgAction, Command};
use owner::Owner;
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

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
    let paths = find_files(&config.paths, config.show_hidden)?;
    if config.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut results = Vec::new();

    for path in paths {
        match fs::metadata(path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    for entry in fs::read_dir(path)? {
                        let p = entry?.path();
                        let is_hidden = p
                            .file_name()
                            .map_or(false, |s| s.to_string_lossy().starts_with('.'));

                        if !is_hidden || show_hidden {
                            results.push(p);
                        }
                    }
                } else {
                    results.push(PathBuf::from(path));
                }
            }
            Err(e) => eprintln!("{}: {}", path, e),
        }
    }

    Ok(results)
}

fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    //         1   2     3     4     5     6     7     8
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = path.metadata()?;

        let uid = metadata.uid();
        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| uid.to_string());

        let gid = metadata.gid();
        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| gid.to_string());

        let file_type = if path.is_dir() { "d" } else { "-" };
        let perms = format_mode(metadata.mode());
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

        table.add_row(
            Row::new()
                .with_cell(file_type) // 1
                .with_cell(perms) // 2
                .with_cell(metadata.nlink()) // 3
                .with_cell(user) // 4
                .with_cell(group) // 5
                .with_cell(metadata.len()) // 6
                .with_cell(modified.format("%b %d %y %H:%M")) // 7
                .with_cell(path.display()), // 8
        );
    }

    Ok(format!("{}", table))
}

fn format_mode(mode: u32) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, Owner::User),
        mk_triple(mode, Owner::Group),
        mk_triple(mode, Owner::Other),
    )
}

fn mk_triple(mode: u32, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_files() {
        // Find all non-hidden entries in a directory
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );

        // Any existing file should be found even if hidden
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // Test multiple path arguments
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        // Find all entries in a directory including hidden
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
    }
}
