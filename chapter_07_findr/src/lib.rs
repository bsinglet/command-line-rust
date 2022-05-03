use crate::EntryType::*;
use clap::{App, Arg};
use walkdir::WalkDir;
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .possible_values(&["f", "d", "l"])
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let names = matches
        .values_of_lossy("names")
        // Options::map will only be used if names are provided
        .map(|vals| {
            vals.into_iter()
                .map(|name| {
                    // try to turn each filename pattern into a valid regex
                    Regex::new(&name)
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                        })
                .collect::<Result<Vec<_>, _>>()
        })
        // Turn the Option(Result) into a Result(Option)
        .transpose()?
        // unwrap the Result, or return an empty vector
        .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|val| match val.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    // clap will already prevent this, but Rust insists that matches covers
                    // every outcome
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
}
