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
    repeated: bool,
    unique: bool,
    skip_fields: usize,
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
        .arg(
            Arg::with_name("repeated")
                .short("d")
                .long("repeated")
                .help("Only show repeated lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("unique")
                .short("u")
                .long("unique")
                .help("Only show unique lines")
                .takes_value(false)
                .conflicts_with("repeated"),
        )
        .arg(
            Arg::with_name("skip-fields")
                .short("f")
                .long("skip-fields")
                .value_name("skip-fields")
                .help("Skip the first N fields")
                .default_value("0"),
        )
        .get_matches();

    let skip_fields = matches
        .value_of("skip-fields")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal field count -- {}", e))?;

    Ok(Config {
        // there are four different ways to do this, all of them equally good:
        //in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
        //in_file: matches.value_of_lossy("in_file").map(String::from).unwrap(),
        //in_file: matches.value_of_lossy("in_file").map(|v| v.into()).unwrap(),
        in_file: matches.value_of_lossy("in_file").map(Into::into).unwrap(),
        out_file: matches.value_of("out_file").map(|v| v.to_string()),
        count: matches.is_present("count"),
        repeated: matches.is_present("repeated"),
        unique: matches.is_present("unique"),
        skip_fields: skip_fields.unwrap(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut unique_lines: Vec<String> = Vec::new();
    let mut line_counts: Vec<i32> = Vec::new();
    let mut previous_line = String::new();
    let mut line = String::new();
    let mut current_count = -1;

    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut my_write = |count: i32, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    loop {
        // read each raw line, including Windows or Linux line-endings
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        // if this is the first line of the file, we have to initialize it this way
        if current_count == -1 {
            previous_line = line.clone();
            current_count = 1;
        // line uniqueness doesn't depend on line endings, even though we have to preserve them in the output.
        } else if config.skip_fields > 0 {
            if line.split_whitespace().skip(config.skip_fields).collect::<String>().trim_end() != previous_line.split_whitespace().skip(config.skip_fields).collect::<String>().trim_end() {
                unique_lines.push(previous_line);
                // store the count of the previous line
                line_counts.push(current_count);
                previous_line = line.clone();
                current_count = 1;
            }
        } else if line.trim_end() != previous_line.trim_end() {
            unique_lines.push(previous_line);
            // store the count of the previous line
            line_counts.push(current_count);
            previous_line = line.clone();
            current_count = 1;
        } else {
            current_count += 1;
        }
            line.clear();
    }
    // store the count of the final repeating line, as long as the entire file isn't empty
    if current_count > -1 {
        unique_lines.push(previous_line);
        line_counts.push(current_count);
    }

    // write out the unique lines, with frequency counts if necessary
    for line_number in 0..unique_lines.len() {
        // with --repeated, only show lines that appeared 2 or more times in a row
        if config.repeated {
            if line_counts[line_number] < 2 {
                continue;
            }
        // with --unique, only show lines that appeared once in a row
        } else if config.unique {
            if line_counts[line_number] != 1 {
                continue;
            }
        }
        my_write(
            line_counts[line_number],
            &unique_lines[line_number]
        )?;
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n >= 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}
