use std::{
    env,
    process,
    error::Error
};

use rust_grep::Config;

/*
 *  Planning
 *  1. Read the user arguments
 *  2. Open the specified file
 *  3. Run the regex
 *  4. Return the output
 */

// Program entry point
fn main() {
    let args: Vec<String> = env::args().collect();  // getting cmd line args

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Parsing error: {}", err);
        process::exit(1)
    });

    println!("Searching for {} in {}", config.query, config.file_name);

    if let Err(x) = run(config) {
        println!("Application Error: {}", x);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = config.read_file();

    dbg!(content);
    Ok(())
}



