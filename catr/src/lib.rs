use clap::{App, Arg};
use std::{
    error::Error,
    fs,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

/// Registers the application with clap, and takes in the specified arguments.
/// Use -h to read the description of the application.
pub fn clap_set() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Bach")
        .about("Rust cat")
        .arg(
            Arg::with_name("file_name")
                .value_name("FILE_NAME")
                .help("File names")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("numbered")
                .short("n")
                .long("number")
                .help("Print line numbers")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("numbered_noblank")
                .short("b")
                .long("number-nonblank")
                .help("Print line numbers on non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        // Safe operation due to default value
        files: matches.values_of_lossy("file_name").unwrap(),
        number_lines: matches.is_present("numbered"),
        number_nonblank_lines: matches.is_present("numbered_noblank"),
    })
}

/// Open the input path. If input is "-", then read from stdin instead of a
/// file
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(filename)?))),
    }
}

fn print_numbered_line(count: usize, line: &str) {
    println!("     {}\t{}", count, line);
}

/// From an IO buffer, does the corresponding cat print based on the config struct
fn print_buffer(config: &Config, rd_buffer: Box<dyn BufRead>) {
    let lines = rd_buffer.lines();
    if config.number_lines {
        lines.fold(1, |acc, ele| {
            print_numbered_line(acc, ele.expect("Line should pass").as_str());
            acc + 1
        });
    } else if config.number_nonblank_lines {
        lines.fold(1, |acc, ele| {
            let cur_line = ele.expect("Line read should be successful");
            if cur_line.is_empty() {
                println!("{}", cur_line);
                acc
            } else {
                print_numbered_line(acc, cur_line.as_str());
                acc + 1
            }
        });
    } else {
        lines.for_each(|ele| {
            println!("{}", ele.expect("should see the next line"));
        });
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let files_iter = config.files.iter();
    for filename in files_iter {
        match open(filename) {
            Ok(x) => print_buffer(&config, x),
            Err(e) => {
                eprint!("Failed to open {}: {}", filename, e);
            }
        }
    }

    Ok(())
}
