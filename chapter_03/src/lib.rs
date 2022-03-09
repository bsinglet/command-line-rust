use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    show_ends: bool,
    squeeze_blank: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf) => {
                let mut last_num = 0;
                let mut previous_line_blank = false;
                let line_ending = if config.show_ends {"$"} else {""};
                for (line_number, each_line) in buf.lines().enumerate() {
                    let each_line = each_line?;
                    // skip consecutive blank lines with -s
                    if config.squeeze_blank && each_line.is_empty() && previous_line_blank {
                                continue;
                    }
                    if each_line.is_empty() {
                        previous_line_blank = true;
                    }else {
                        previous_line_blank = false;
                    }
                    if config.number_lines {
                        println!("{:>6}\t{}{}", line_number + 1, each_line, line_ending);
                    }else if config.number_nonblank_lines {
                        if !each_line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}{}", last_num, each_line, line_ending);
                        }else {
                            println!("{}", line_ending);
                        }
                    }else {
                        println!("{}{}", each_line, line_ending);
                    }
                }
            },
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("show_ends")
                .short("E")
                .long("show-ends")
                .help("Display $ at end of each line")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("squeeze_blank")
                .short("s")
                .long("squeeze-blank")
                .help("Suppress repeated empty output lines")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
        show_ends: matches.is_present("show_ends"),
        squeeze_blank: matches.is_present("squeeze_blank"),
    })
}
