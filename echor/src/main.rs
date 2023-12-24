use clap::App;

fn main() {
    // This section is just registering the application as a commandline one.
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Bach")
        .about("Rust commandline suite, echo")
        .get_matches();
}
