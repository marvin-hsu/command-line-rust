use std::error::Error;

use chrono::NaiveDate;
use clap::Command;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("calr")
        .author("marvinhsu")
        .about("Rust cal")
        .version("0.1.0")
        .get_matches();

    Ok(Config {
        month: todo!(),
        year: todo!(),
        today: todo!(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
