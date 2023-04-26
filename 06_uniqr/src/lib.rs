use clap::Command;
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
        .get_matches();

    Ok(Config {
        in_files: "".to_string(),
        out_files: None,
        count: false,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}
