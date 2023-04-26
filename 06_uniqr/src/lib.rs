use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

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
    let mut file = open(&config.in_files).map_err(|e| format!("{}: {}", config.in_files, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_files {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count = 0_u64;

    loop {
        let bytes = file.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;

        line.clear();
    }

    print(count, &previous)?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
