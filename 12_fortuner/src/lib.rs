use std::{
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::{Arg, ArgAction, Command};
use rand::{rngs::StdRng, seq::SliceRandom, RngCore, SeedableRng};
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    source: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

#[derive(Debug)]
pub struct Fortune {
    source: String,
    text: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("fortuner")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust fortune")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input files or directories")
                .action(ArgAction::Append)
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("insensitive")
                .help("Case-insensitive pattern matching")
                .action(ArgAction::SetTrue)
                .short('i')
                .long("insensitive"),
        )
        .arg(
            Arg::new("pattern")
                .help("Pattern")
                .short('m')
                .long("pattern")
                .value_name("PATTERN"),
        )
        .arg(
            Arg::new("seed")
                .help("Random seed")
                .short('s')
                .long("seed")
                .value_name("SEED"),
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .map(|s| {
            RegexBuilder::new(s)
                .case_insensitive(matches.get_flag("insensitive"))
                .build()
                .map_err(|_| format!("Invalid --pattern \"{}\"", s))
        })
        .transpose()?;

    let seed = matches
        .get_one::<String>("seed")
        .map(|s| {
            s.parse::<u64>()
                .map_err(|_| format!("\"{}\" not a valid integer", s))
        })
        .transpose()?;

    Ok(Config {
        source: matches
            .get_many::<String>("file")
            .unwrap()
            .cloned()
            .collect(),
        pattern,
        seed,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.source)?;
    let fortunes = read_fortunes(&files)?;
    if let Some(pattern) = config.pattern {
        let mut prev_source = None;
        for fortune in fortunes.iter().filter(|f| pattern.is_match(&f.text)) {
            if prev_source.as_ref().map_or(true, |s| s != &fortune.source) {
                eprintln!("({})\n%", fortune.source.clone());
                prev_source = Some(fortune.source.clone());
            }
            println!("{}\n%", fortune.text);
        }
    } else {
        println!(
            "{}",
            pick_fortune(&fortunes, config.seed)
                .or_else(|| Some("No fortunes found".to_string()))
                .unwrap()
        )
    }
    Ok(())
}

pub fn find_files(paths: &[String]) -> MyResult<Vec<PathBuf>> {
    let dat = OsStr::new("dat");
    let mut files = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => return Err(format!("{}: {}", path, e).into()),
            Ok(_) => files.extend(
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| e.file_type().is_file() && e.path().extension() != Some(dat))
                    .map(|e| e.path().into()),
            ),
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

pub fn read_fortunes(paths: &[PathBuf]) -> MyResult<Vec<Fortune>> {
    let mut fortunes = vec![];
    let mut buffer = vec![];

    for path in paths {
        let basename = path.file_name().unwrap().to_string_lossy().into_owned();
        let file = File::open(path)
            .map_err(|e| format!("{}: {}", path.to_string_lossy().into_owned(), e))?;

        for line in BufReader::new(file).lines().filter_map(Result::ok) {
            if line == "%" {
                if !buffer.is_empty() {
                    fortunes.push(Fortune {
                        source: basename.clone(),
                        text: buffer.join("\n"),
                    });
                    buffer.clear();
                }
            } else {
                buffer.push(line.to_string());
            }
        }
    }

    Ok(fortunes)
}

pub fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    let mut rng: Box<dyn RngCore> = if let Some(val) = seed {
        Box::new(StdRng::seed_from_u64(val))
    } else {
        Box::new(rand::thread_rng())
    };

    fortunes.choose(&mut rng).map(|f| f.text.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{find_files, pick_fortune, read_fortunes, Fortune};

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );

        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        // Test for multiple sources, path must be unique and sorted
        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }

    #[test]
    fn test_read_fortunes() {
        // Parses all the fortunes without a filter
        let res = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 6);
            assert_eq!(
                fortunes.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
                A. Collared greens."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\n\
                A: A bad idea (bad-eye deer)."
            );
        }

        // Filters for matching text
        let res = read_fortunes(&[
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 11);
    }

    #[test]
    fn test_pick_fortune() {
        // Create a slice of fortunes
        let fortunes = &[
            Fortune {
                source: "fortunes".to_string(),
                text: "You cannot achieve the impossible without \
                      attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Assumption is the mother of all screw-ups.".to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];

        // Pick a fortune with a seed
        assert_eq!(
            pick_fortune(fortunes, Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
