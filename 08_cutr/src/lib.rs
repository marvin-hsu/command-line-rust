use clap::{Arg, ArgAction, Command};
use std::{error::Error, num::NonZeroUsize, ops::Range};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("cutr")
        .version("1.0.1")
        .author("marvinhsu")
        .about("Rust cut")
        .arg(
            Arg::new("files")
                .default_value("-")
                .action(ArgAction::Append)
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(0..),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('b')
                .long("bytes")
                .help("Selected bytes"),
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .short('c')
                .long("chars")
                .help("Selected characters"),
        )
        .arg(
            Arg::new("Delim")
                .value_name("DELIMITER")
                .short('d')
                .long("delim")
                .default_value("")
                .help("Field delimiter"),
        )
        .arg(
            Arg::new("fields")
                .value_name("FIELDS")
                .short('f')
                .long("fields")
                .help("Selected fields"),
        )
        .get_matches();

    Ok(Config {
        files: todo!(),
        delimiter: todo!(),
        extract: todo!(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    range
        .split(',')
        .map(|val| {
            let parse_into_usize = |n: &str| -> Result<usize, String> {
                let n: usize = n
                    .parse::<NonZeroUsize>()
                    .map_err(|_| format!("illegal list value: \"{}\"", n))?
                    .into();
                Ok(n)
            };

            match val.split('-').collect::<Vec<&str>>().as_slice() {
                [n] => {
                    let n = parse_into_usize(n)?;
                    Ok((n - 1)..n)
                }
                [n1, n2] => {
                    let n1: usize = parse_into_usize(n1)?;
                    let n2: usize = parse_into_usize(n2)?;
                    Ok((n1 - 1)..n2)
                }
                _ => Err(format!("illegal list value: \"{}\"", val)),
            }
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}

#[cfg(test)]
mod unit_tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos_success_input_1() {
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);
    }

    #[test]
    fn test_parse_pos_success_input_split_by_comma() {
        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
    }

    #[test]
    fn test_parse_pos_success_input_split_by_dash() {
        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);
    }

    #[test]
    fn test_parse_pos_success_input_split_by_comma_and_dash() {
        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }

    #[test]
    fn test_parse_pos_fail_input_empty() {
        assert!(parse_pos("").is_err());
    }

    #[test]
    fn test_parse_pos_fail_input_zero() {
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"");

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"");
    }
}
