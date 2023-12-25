use clap::{App, Arg};
use std::{error::Error, fs};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn clap_set() -> clap::ArgMatches<'static> {
    App::new("catr").version("0.1.0").author("Bach").about("Rust cat")
        .arg(
            Arg::with_name("file_name")
            .value_name("FILE_NAME")
            .help("File names")
            .required(true)
            .min_values(1),
            )
        .arg(
            Arg::with_name("numbered").short("n").help("Print line numbers")
            .takes_value(false),
            )
        .arg(
            Arg::with_name("numbered_noblank").short("b")
            .help("Print line numbers on non-blank lines")
            .takes_value(false),
            )
        .get_matches()
}

pub fn run() -> MyResult<()> {
    let matches = clap_set();

    let file_names = matches.values_of_lossy("file_name").unwrap();

    file_names.into_iter().map(|ele| fs::read_to_string(&ele).unwrap())
        .for_each(|content| print!("{content}"));
    Ok(())
}
