use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust grep")
        .get_matches();

    Config {}
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
