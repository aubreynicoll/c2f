use std::env;
use std::process;

use c2f::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = c2f::run(&config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
