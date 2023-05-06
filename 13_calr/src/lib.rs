use std::error::Error;

use chrono::NaiveDate;
use clap::{Arg, ArgAction, Command};

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
        .arg(
            Arg::new("show_current_year")
                .short('y')
                .long("year")
                .help("Show whole current year")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("month")
                .short('m')
                .value_name("Month")
                .value_parser(1..12)
                .help("Month name or number"),
        )
        .arg(
            Arg::new("year")
                .value_name("YEAR")
                .help("Year")
                .value_parser(1..9999)
                .num_args(1),
        )
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
