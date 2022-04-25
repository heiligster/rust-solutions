#![allow(unused)]
use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Structure to hold configuration values as well as file paths to operate on
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Parses command line arguments and options and returns them as a Config
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Philipp Mueller <heiligster@gmail.com>")
        .about("A super simple cat clone written in Rust")
        .arg(
            Arg::new("files")
                .takes_value(true)
                .value_name("FILES")
                .help("Input file(s)")
                .multiple_values(true)
                .default_value("-")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .takes_value(false)
                .help("Add line numbers")
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .takes_value(false)
                .help("Add line numbers to non empty lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for file in &config.files {
        match open(&file) {
            Ok(reader) => {
                print(&config, reader);
            }
            Err(e) => eprintln!("Failed to open {}: {}", file, e),
        }
    }

    Ok(())
}

fn print(config: &Config, reader: impl BufRead) -> MyResult<()> {
    let mut last_num = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if config.number_nonblank_lines {
            if !line.is_empty() {
                last_num += 1;
                println!("{:>6}\t{}", last_num, line);
            } else {
                println!();
            }
        } else if config.number_lines {
            println!("{:>6}\t{}", i + 1, line);
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
