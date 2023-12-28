fn main() {

    // Function return unwrapping
    if let Err(e) = catr::clap_set().and_then(catr::run) {
        catr::print_and_exit(e)
    }
}
