use std::env;
use std::process;

use game::Config;

fn main() {
    let config = Config::parse(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = game::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
