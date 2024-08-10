fn main() {
    if let Err(e) = catr::clap_set().and_then(catr::run) {
        eprintln!("Error found: {}", e);
        std::process::exit(1)
    }
}
