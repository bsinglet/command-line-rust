use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Show counts")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        // there are four different ways to do this, all of them equally good:
        //in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
        //in_file: matches.value_of_lossy("in_file").map(String::from).unwrap(),
        //in_file: matches.value_of_lossy("in_file").map(|v| v.into()).unwrap(),
        in_file: matches.value_of_lossy("in_file").map(Into::into).unwrap(),
        out_file: matches.value_of("out_file").map(|v| v.to_string()),
        count: matches.is_present("count")
    })
}

pub fn run(config: Config) -> MyResult<()> {
    //println!("{:?}", config);
    let mut unique_lines: Vec<String> = Vec::new();
    let mut line_counts: Vec<i32> = Vec::new();
    let mut previous_line: String = "".to_string();
    let mut current_count = -1;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    match open(&config.in_file) {
        Err(err) => eprintln!("{}: {}", &config.in_file, err),
        Ok(file) => {
            for each_line in file.lines() {
                let line = each_line.unwrap();
                if current_count == -1 {
                    previous_line = line.clone();
                    current_count = 1;
                } else if line != previous_line {
                    unique_lines.push(previous_line);
                    // store the count of the previous line
                    line_counts.push(current_count);
                    previous_line = line.clone();
                    current_count = 1;
                } else {
                    current_count += 1;
                }
            }
            if current_count > -1 {
                unique_lines.push(previous_line);
                line_counts.push(current_count);
            }

            for line_number in 0..unique_lines.len() {
                if config.count {
                    writeln!(out_file, "{} {}",
                        format!("{:>4}", line_counts[line_number]),
                        unique_lines[line_number]);
                } else {
                    writeln!(out_file, "{}", unique_lines[line_number]);
                }
            }
        }
    };

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
