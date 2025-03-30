use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, Command};

type GenError = Box<dyn Error>;
type GenResult<T> = Result<T, GenError>;

#[allow(dead_code)] // TODO delete this
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> GenResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Alexander Eremenchuk")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-")
                .hide_default_value(true),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        number_lines: false,
        number_nonblank_lines: false,
    })
}

fn open(filename: &str) -> GenResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> GenResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                let mut line = String::new();
                while file.read_line(&mut line)? != 0 {
                    print!("{}", line);
                    line.clear();
                }
            }
        }
    }
    Ok(())
}
