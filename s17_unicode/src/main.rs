fn main() {
    if let Err(e) = s17_unicode::get_args().and_then(s17_unicode::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
