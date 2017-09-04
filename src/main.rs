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

use rand::Rng;
use rand::distributions::Range;

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

    let tuning: Vec<Note> = if let Some(input) = matches.value_of("tuning") {
        parse_tuning(&String::from(input))?
    } else {
        vec![Note::E, Note::A, Note::D, Note::G, Note::B, Note::E]
    };

    Ok(())
}

fn parse_tuning(input: &String) -> Result<Vec<Note>> {
    let input: Vec<String> = input.split(',')
        .map(|i| i.replace(|j| j == ' ', ""))
        .collect();

    let mut notes: Vec<Note> = Vec::new();

    for note in input.into_iter() {
        // TODO: Extract this into something more sensible
        // If any of the characters are Q, quit the program
        if note == "Q" || note == "q" {
            ::std::process::exit(0);
        }

        notes.push(Note::try_from_string(&note)?);
    }

    Ok(notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tuning() {
        struct Test {
            input: String,
            expected: Vec<Note>,
        }

        let tests = [
            Test {
                input: String::from("E, A, D, G, B, E"),
                expected: vec![Note::E, Note::A, Note::D, Note::G, Note::B, Note::E],
            },
            Test {
                input: String::from("D#, A, D#, Gb, B"),
                expected: vec![Note::DSharp, Note::A, Note::DSharp, Note::FSharp, Note::B],
            },
            Test {
                input: String::from("B, A, D"),
                expected: vec![Note::B, Note::A, Note::D],
            }
        ];

        for test in &tests {
            assert_eq!(test.expected, parse_tuning(&test.input).unwrap());
        }
    }
}