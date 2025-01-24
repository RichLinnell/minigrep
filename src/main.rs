//! # minigrep
//! Minigrep is a worked example from the Rust documentation that is a very lightweight 
//! version of the grep tool found in linux.  It takes two parameters, a search string, and a filename
//! it then proceeds to look for all rows in the file that contain the provided search string.
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
