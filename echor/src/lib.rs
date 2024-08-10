use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// This function registers the command line program, and provides a help
/// argument flag.
fn cmd_args() -> clap::ArgMatches<'static> {
    App::new("echor")
        .version("0.1.0")
        .author("Bach")
        .about("Rust commandline suite, echo")
        .arg(
            Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
            )
        .arg(
            Arg::with_name("omit_newline")
            .short("n")
            .help("Do not print trailing newline")
            .takes_value(false),
            )
        .get_matches()
}

pub fn run() -> MyResult<()> {
    let matches = cmd_args();

    let output = matches.values_of_lossy("text").unwrap();

    print!("{}{}", output.join(" "),
        if matches.is_present("omit_newline") {""} else {"\n"});

    Ok(())
}
