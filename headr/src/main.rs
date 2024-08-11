use anyhow::Result;
use clap::Parser;
use std::{
    convert::From,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Parser)]
#[command(name = "headr")]
#[command(version = "0.0.1")]
#[command(
    about = "Rust version of the POSIX head program",
    long_about = "
Print the first 10 lines of each FILE to standard output.
With more than one FILE, precede each with a header giving the file name.
"
)]
pub struct Cli {
    #[arg(default_value = "-", value_name = "FILE")]
    file: Vec<String>,

    /// print the first NUM lines instead of the first 10; with the leading
    /// '-', print all but the last NUM lines of each file. Overriden by -c
    #[arg(short = 'n', long, value_name = "LINES", default_value = "10", value_parser=clap::value_parser!(u64).range(1..))]
    lines: u64,

    /// Print the first K bytes of each file; with the leading '-', print
    /// all but the last K bytes of each file
    #[arg(short = 'c', long, value_name = "BYTES", conflicts_with = "lines", value_parser=clap::value_parser!(u64).range(1..))]
    bytes: Option<u64>,

    /// Will never print the headers giving file names
    #[arg(short, long, visible_alias = "silent")]
    quiet: bool,

    /// Always print the headers giving file names
    #[arg(short, long)]
    verbose: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filename)?))),
    }
}

/// Prints out [num_lines] number of lines out of the [reader] file handle.
fn read_lines(num_lines: u64, mut reader: Box<dyn BufRead>) -> Result<()> {
    let mut buf = String::new();
    for _ in 0..num_lines {
        let n = reader.read_line(&mut buf)?;
        if n == 0 {
            break;
        } else {
            print!("{}", buf);
            buf.clear();
        }
    }
    Ok(())
}

/// Takes the number of bytes to read, and a file handle, and print those
/// bytes out as a string.
fn read_bytes(num_bytes: u64, reader: Box<dyn BufRead>) -> Result<()> {
    let mut limit_read = reader.take(num_bytes);
    let mut buf: Vec<u8> = vec![0; num_bytes as usize];
    let bytes_read = limit_read.read(&mut buf)?;
    let output = String::from_utf8_lossy(&buf[..bytes_read]);
    print!("{}", output);
    Ok(())
}

/// Does the flow control logic on which functionality will be used on a single file
fn head_program(config: &Cli, reader: Box<dyn BufRead>) -> Result<()> {
    // We will be running the bytes mode if that flag has been touched, else
    // run the lines mode
    if let Some(num_bytes) = config.bytes {
        read_bytes(num_bytes, reader)?;
    } else {
        read_lines(config.lines, reader)?;
    }
    Ok(())
}

fn run_file(config: &Cli, filename: &str) -> Result<()> {
    match open(filename) {
        Err(err) => eprintln!("{}: {}", filename, err),
        Ok(reader) => head_program(config, reader)?,
    }
    Ok(())
}

fn run(config: Cli) -> Result<()> {
    let num_files = config.file.len() - 1;
    let files = config.file.iter();
    let header: bool = match num_files {
        0 => config.verbose,
        _ => !config.quiet,
    };
    if header {
        for (i, filename) in files.enumerate() {
            println!("==> {} <==", filename);
            run_file(&config, filename)?;
            if i < num_files {
                println!();
            }
        }
    } else {
        for filename in files {
            run_file(&config, filename)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run(Cli::parse()) {
        eprintln!("{}", err);
        std::process::exit(1)
    }
}
