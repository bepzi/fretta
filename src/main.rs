extern crate clap;
extern crate error_chain;
extern crate rand;

mod notes;

use clap::{App, Arg};

use notes::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = "fretta";

fn main() {
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
}
