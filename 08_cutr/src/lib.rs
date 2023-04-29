use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {}

pub fn get_args() -> MyResult<Config> {
    Ok(Config {})
}

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}
