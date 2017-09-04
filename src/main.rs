// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate rand;

mod notes;
use notes::*;

mod errors {
    error_chain! { }
}
use errors::*;

use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = "fretta";

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = App::new(APP_NAME)
        .version(VERSION)
        .about("Tool for training one's knowledge of the fretboard")
        .arg(Arg::with_name("tuning")
            .short("t")
            .long("tuning")
            .value_name("TUNING")
            .help("Comma-separated list of notes representing the tuning of the instrument")
            .takes_value(true))
        .get_matches();

    Ok(())
}