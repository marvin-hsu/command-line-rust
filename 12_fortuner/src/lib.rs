use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    source: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config {
        source: todo!(),
        pattern: todo!(),
        seed: todo!(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
