use clap::{
    App,
    Arg
};

fn main() {
    // This section is just registering the application as a commandline one.
    let matches = App::new("echor")
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
        .get_matches();

    println!("{:?}",matches);
}
