use std::{
    env,
    process,
};

use rust_grep::{
    Config,
    run
};

/*
 *  Planning
 *  1. Read the user arguments (Done!)
 *  2. Open the specified file (Done!)
 *  3. Run the regex
 *  4. Return the output
 */

// Program entry point
fn main() {
    let args: Vec<String> = env::args().collect();  // getting cmd line args

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Parsing error: {}", err);
        process::exit(1)
    });

    if let Err(x) = run(config) {   // Running the main program here.
        eprintln!("Application Error: {}", x);
        process::exit(1)
    };
}




