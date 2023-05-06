use std::{error::Error, str::FromStr};

use chrono::{Datelike, Local, NaiveDate};
use clap::{Arg, ArgAction, Command};
use itertools::Itertools;

const MONTH_NAMES: [&str; 12] = [
    "january",
    "gebruary",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];

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
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["month","year"]),
        )
        .arg(
            Arg::new("month")
                .short('m')
                .value_name("MONTH")
                .help("Month name or number"),
        )
        .arg(
            Arg::new("year")
                .value_name("YEAR")
                .help("Year")
                .num_args(1),
        )
        .get_matches();

    let mut month = matches
        .get_one::<String>("month")
        .map(|s| parse_month(s))
        .transpose()?;
    let mut year = matches
        .get_one::<String>("year")
        .map(|s| parse_year(s))
        .transpose()?;
    let today = Local::now();
    if matches.get_flag("show_current_year") {
        month = None;
        year = Some(today.year())
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year())
    }

    Ok(Config {
        month,
        year: year.unwrap_or_else(|| today.year()),
        today: today.date_naive(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    val.parse()
        .map_err(|_| format!("Invalid integer \"{}\"", val).into())
}

pub fn parse_year(year: &str) -> MyResult<i32> {
    parse_int(year).and_then(|num| {
        if (1..=9999).contains(&num) {
            Ok(num)
        } else {
            Err(format!("year \"{}\" not in the range 1 through 9999", year).into())
        }
    })
}

pub fn parse_month(month: &str) -> MyResult<u32> {
    match parse_int(month) {
        Ok(num) => {
            if (1..=12).contains(&num) {
                Ok(num)
            } else {
                Err(format!("month \"{}\" not in the range 1 through 12", num).into())
            }
        }
        _ => {
            let lower = month.to_lowercase();
            match MONTH_NAMES
                .iter()
                .enumerate()
                .find(|v| v.1.starts_with(&lower))
            {
                Some(m) => {
                    println!("{}", m.1);
                    Ok(m.0 as u32 + 1)
                }
                None => Err(format!("Invalid month \"{}\"", month).into()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        // Parse positive int as usize
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        // Parse negative int as i32
        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        // Fail on a string
        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_year() {
        let res = parse_year("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1i32);

        let res = parse_year("9999");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 9999i32);

        let res = parse_year("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"0\" not in the range 1 through 9999"
        );

        let res = parse_year("10000");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"10000\" not in the range 1 through 9999"
        );

        let res = parse_year("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1 through 12"
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1 through 12"
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }
}
