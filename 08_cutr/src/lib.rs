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
            let r: usize = val.parse::<NonZeroUsize>().map_err(|_| val)?.into();
            Ok(r-1..r)
        })
        .collect()
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
    fn test_parse_pos_success_input_1_comma_3() {
        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
    }
}
