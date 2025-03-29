use std::error::Error;

use clap::Command;

type GenError = Box<dyn Error>;
type GenResult<T> = Result<T, GenError>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> GenResult<Config> {
    let _matches = Command::new("catr")
        .version("0.1.0")
        .author("Alexander Eremenchuk")
        .about("Rust cat")
        .get_matches();

    Ok(Config {
        files: vec![String::from("-")],
        number_lines: false,
        number_nonblank_lines: false,
    })
}

pub fn run(config: Config) -> GenResult<()> {
    dbg!(config.files);
    dbg!(config.number_lines);
    dbg!(config.number_nonblank_lines);
    Ok(())
}
