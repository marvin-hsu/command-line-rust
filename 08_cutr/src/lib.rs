use clap::{Arg, ArgAction, Command};
use csv::StringRecord;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    ops::Range,
};

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
                .help("Selected bytes")
                .conflicts_with_all(["chars", "fields"]),
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .short('c')
                .long("chars")
                .help("Selected characters")
                .conflicts_with_all(["fields", "bytes"]),
        )
        .arg(
            Arg::new("fields")
                .value_name("FIELDS")
                .short('f')
                .long("fields")
                .help("Selected fields")
                .conflicts_with_all(["chars", "bytes"]),
        )
        .arg(
            Arg::new("delimiter")
                .value_name("DELIMITER")
                .short('d')
                .long("delim")
                .default_value("")
                .help("Field delimiter")
                .default_value("\t"),
        )
        .get_matches();

    let delimiter = matches.get_one::<String>("delimiter").unwrap();
    let delim_bytes = delimiter.as_bytes();
    if delim_bytes.len() != 1 {
        return Err(From::from(format!(
            "--delim \"{}\" nust be a single byte",
            delimiter
        )));
    }

    let fields = matches
        .get_one::<String>("fields")
        .map(|s| parse_pos(s))
        .transpose()?;
    let bytes = matches
        .get_one::<String>("bytes")
        .map(|s| parse_pos(s))
        .transpose()?;
    let chars = matches
        .get_one::<String>("chars")
        .map(|s| parse_pos(s))
        .transpose()?;

    let extract = if let Some(fields_pos) = fields {
        Extract::Fields(fields_pos)
    } else if let Some(bytes_pos) = bytes {
        Extract::Bytes(bytes_pos)
    } else if let Some(chars_pos) = chars {
        Extract::Chars(chars_pos)
    } else {
        return Err(From::from("Must have --fields, --bytes, or --chars"));
    };

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        delimiter: *delim_bytes.first().unwrap(),
        extract,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Ok(_) => println!("Opened {}", filename),
            Err(e) => eprintln!("{}: {}", filename, e),
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    range
        .split(',')
        .map(|val| {
            let parse_into_usize = |input: &str| -> Result<usize, String> {
                let value_error = format!("illegal list value: \"{}\"", range);

                if input.starts_with('+') {
                    Err(value_error)
                } else {
                    input
                        .parse::<NonZeroUsize>()
                        .map(usize::from)
                        .map_err(|_| value_error)
                }
            };
            match val.split('-').collect::<Vec<&str>>().as_slice() {
                [n] => {
                    let n = parse_into_usize(n)?;
                    Ok((n - 1)..n)
                }
                [n1, n2] => {
                    let n1: usize = parse_into_usize(n1)?;
                    let n2: usize = parse_into_usize(n2)?;
                    if n1 >= n2 {
                        Err(format!(
                            "First number in range ({}) must be lower than second number ({})",
                            n1, n2
                        ))
                    } else {
                        Ok((n1 - 1)..n2)
                    }
                }
                _ => Err(format!("illegal list value: \"{}\"", val)),
            }
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}

fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let chars = line.chars().collect::<Vec<_>>();
    char_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        .collect()
}

fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = byte_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

fn extract_fields<'a>(record: &'a StringRecord, fileld_pos: &[Range<usize>]) -> Vec<&'a str> {
    fileld_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}

#[cfg(test)]
mod unit_tests {
    use csv::StringRecord;

    use super::{extract_bytes, extract_chars, extract_fields, parse_pos};

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
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0-1\"");
    }

    #[test]
    fn test_parse_pos_fail_input_plus() {
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1\"");

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1-2\"");

        let res = parse_pos("1-+2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-+2\"");
    }

    #[test]
    fn test_parse_pos_fail_input_alphabet() {
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"");

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1,a\"");

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-a\"");

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a-1\"");
    }

    #[test]
    fn test_parse_pos_fail_input_not_clear_range() {
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_pos_fail_input_bondary_error() {
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[0..1]), "".to_string());
        assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 1..2, 4..5]), "áb".to_string());
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(extract_fields(&rec, &[0..1, 2..3]), &["Captain", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    }
}
