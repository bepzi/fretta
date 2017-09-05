// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate rand;

mod note;
use note::*;

mod errors {
    error_chain! { }
}
use errors::*;

use clap::{App, Arg};

use rand::distributions::{Range, IndependentSample};

use std::io;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = "fretta";

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

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

    // Retrieve the tuning option from the -t flag, or the default tuning of EADGBE
    let tuning: Vec<Note> = if let Some(input) = matches.value_of("tuning") {
        parse_tuning(&String::from(input))?
    } else {
        vec![Note::E, Note::A, Note::D, Note::G, Note::B, Note::E]
    };

    let num_strings = tuning.len();

    // Create the RNG needed to pick a random string and fret
    let mut rng = rand::thread_rng();
    let string_range = Range::new(0, num_strings);
    let fret_range = Range::new(1, 23);

    'main: loop {
        // Pick a random string and fret
        let string = string_range.ind_sample(&mut rng);
        let fret = fret_range.ind_sample(&mut rng);

        // Repeat until the input isn't an empty string
        let mut input;
        'input: loop {
            println!("{} string, fret {}", tuning[string], fret);

            // Try to get the answer from the user
            input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            input = input.trim().to_string();

            // Quit once we have an answer
            if !input.is_empty() {
                break 'input;
            }
        }

        // If the user wanted to quit, end the program
        if input.to_lowercase() == "q" {
            println!("Quitting...");
            break 'main;
        }

        let guess = Note::try_from_string(&input)?;
        let answer = calculate_note(tuning[string], fret);

        if guess == answer {
            println!("Correct!\n");
        } else {
            println!("Incorrect! The answer was: {}\n", answer);
        }
    }

    Ok(())
}

/// Converts a comma-separated string of notes into a Vec<Note>, returning an error if any
/// of the inputs can't be parsed.
fn parse_tuning(input: &String) -> Result<Vec<Note>> {
    // Strip out ',' and any whitespace into a vec of potential notes
    let input: Vec<String> = input.split(',')
        .map(|i| i.replace(|j: char| j.is_whitespace(), ""))
        .collect();

    let mut notes: Vec<Note> = Vec::with_capacity(input.len());

    for note in input.into_iter() {
        notes.push(Note::try_from_string(&note)?);
    }

    Ok(notes)
}

/// Given a base note representing a string and a fret on the fretboard, calculates the
/// corresponding note at that fret.
fn calculate_note(base_note: Note, fret: usize) -> Note {
    let mut list: Vec<Note> = Vec::with_capacity(12);

    list.push(base_note);
    for i in 0..11 {
        let prev_note = list[i];
        list.push(prev_note.next());
    }

    // Since there are 12 notes, `fret` % 12 will reduce the index to the corresponding note in
    // the list of notes that were generated from the base note
    list[fret % 12]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_note() {
        struct Test {
            base_note: Note,
            fret: usize,
            expected: Note,
        }

        let tests = [
            Test {
                base_note: Note::A,
                fret: 2,
                expected: Note::B,
            },
            Test {
                base_note: Note::GSharp,
                fret: 4,
                expected: Note::C,
            }
        ];

        for test in &tests {
            assert_eq!(test.expected, calculate_note(test.base_note, test.fret));
        }
    }

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