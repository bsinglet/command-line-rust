use clap::{App, Arg};
use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead, BufReader, Lines, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show character count")
                .takes_value(false)
        )
        .get_matches();

    let mut lines = true;
    let mut words = true;
    let mut bytes = true;
    let mut chars = false;
    if matches.is_present("lines") || matches.is_present("words") || matches.is_present("bytes") || matches.is_present("chars") {
        lines = matches.is_present("lines");
        words = matches.is_present("words");
        bytes = matches.is_present("bytes");
        chars = matches.is_present("chars");
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines,
        words: words,
        bytes: bytes,
        chars: chars,
    })
}

pub fn print_stats(config: Config, lines: usize, words: usize, chars: usize, bytes: usize) {
    match (config.lines, config.words, config.chars, config.bytes) {
        (true, true, true, false) => println!("{} {} {}", lines, words, chars),
        (true, true, false, true) => println!("{} {} {}", lines, words, bytes),
        (true, false, true, false) => println!("{} \t {}", lines, chars),
        (true, false, false, true) => println!("{} \t {}", lines, bytes),
        (false, true, true, false) => println!("\t {} {}", words, chars),
        (false, true, false, true) => println!("\t {} {}", words, bytes),
        (false, false, true, false) => println!("\t \t {}", words),
        (false, false, false, true) => println!("\t \t {}", bytes),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    let mut total_bytes = 0;
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                let mut file_lines = 0;
                let mut file_words = 0;
                let mut file_chars = 0;
                let mut file_bytes = 0;
                for line in file.lines() {
                    file_lines += 1;
                    file_words += line.unwrap().split_whitespace().collect().len();
                    file_chars += line.unwrap().chars().collect().len();
                    file_bytes += line.unwrap().len();
                }
                print_stats(config, file_lines, file_words, file_chars, file_bytes);
                total_lines += file_lines;
                total_words += file_words;
                total_chars += file_chars;
                total_bytes += file_bytes;
            }
        }
    }

    Ok(())
}
