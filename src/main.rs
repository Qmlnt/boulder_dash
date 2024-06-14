use std::env;
use std::process;

use boulder_dash::Arguments;

fn main() {
    let config = Arguments::parse(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = boulder_dash::run(&config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
