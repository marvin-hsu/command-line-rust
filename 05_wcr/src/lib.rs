use std::error::Error;

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .help("Input file(s")
                .default_value("-")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("bytes")
                .long("bytes")
                .short('b')
                .help("Show byte count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .long("chars")
                .short('m')
                .help("Show character count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lines")
                .long("lines")
                .short('l')
                .help("Show line count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .long("words")
                .short('w')
                .help("Show word count")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");

    if [lines, words, bytes, chars].iter().all(|v| !(*v)) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
