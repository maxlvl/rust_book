use std::env;
use std::process;

use tinier_grep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("An error occurred: {}", err);
        process::exit(1);
    });

    if let Err(e) = tinier_grep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
