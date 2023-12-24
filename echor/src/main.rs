

fn main() {
    if let Err(e) = echor::run()     {
        eprint!("{e}");
        std::process::exit(1)
    }
}
