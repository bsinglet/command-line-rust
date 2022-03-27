use clap::{App, Arg, value_t};
use std::error::Error;
use std::fs::File;
//use std::io::{self, BufRead, BufReader};
use std::io::IoSliceMut;
use std::str::FromStr;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .help("print the first K lines instead of the first 10;\nwith the leading '-', print all but the last\nK lines of each file")
                .takes_value(true)
                .multiple(false)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("print the first K bytes of each file;\nwith the leading '-', print all but the last\nK bytes of each file")
                .takes_value(true)
                .multiple(false)
                .conflicts_with("lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: usize::from_str(matches.value_of("lines").unwrap()).unwrap(),
        bytes: Some(usize::from_str(matches.value_of("bytes").unwrap_or_default()).unwrap_or_default()),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    for filename in config.files {
        match File::open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                if Some(config.bytes.unwrap()) {
                    let metadata = file.metadata()?;
                    let file_length: usize = metadata.len() as usize;
                    let buffer_size = match bytes.unwrap() > file_length {
                        true => config.bytes.unwrap(),
                        false => config.bytes.unwrap(),
                    };
                    let mut buffer = [0; buffer_size];
                    let contents = file.read_exact(&mut buffer)?;
                    println!("{:#?}", contents);
                }
            }
        }
    }
    Ok(())
}
