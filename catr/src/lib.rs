use clap::{App, Arg};
use std::{
    error::Error,
    fs,
    io::{self, BufRead, BufReader}
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
    let matches = App::new("catr").version("0.1.0").author("Bach").about("Rust cat")
        .arg(
            Arg::with_name("file_name")
            .value_name("FILE_NAME")
            .help("File names")
            .multiple(true)
            .default_value("-"),
            )
        .arg(
            Arg::with_name("numbered").short("n").help("Print line numbers")
            .takes_value(false).conflicts_with("number_nonblank"),
            )
        .arg(
            Arg::with_name("numbered_noblank").short("b")
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(filename)?))),
    }
}

pub fn print_and_exit(e: Box<dyn Error>) {
    eprintln!("{e}");
    std::process::exit(1);
}

pub fn run(config: Config) -> MyResult<()> {

    for filename in config.files {
        match open(&filename) {
            Ok(x) => x.lines().for_each(|ele| println!("{}", ele.unwrap())),
            Err(e) => {
                eprint!("Failed to open {filename}: ");
                print_and_exit(e)},
        }
    };

    Ok(())
}
