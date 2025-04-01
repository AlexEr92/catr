use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

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
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue)
                .help("Number non-blank lines")
                .conflicts_with("number"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
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
                let mut line_num = 0;
                let mut line_last_number = 0;
                while file.read_line(&mut line)? != 0 {
                    if config.number_lines {
                        line_num += 1;
                        print!("{:>6}\t{}", line_num, line);
                    } else if config.number_nonblank_lines {
                        if line != "\n" {
                            line_last_number += 1;
                            print!("{:>6}\t{}", line_last_number, line);
                        } else {
                            println!();
                        }
                    } else {
                        print!("{}", line);
                    }
                    line.clear();
                }
            }
        }
    }
    Ok(())
}
