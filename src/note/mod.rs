use errors::*;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Note {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

impl Note {
    pub fn try_from_string(s: &String) -> Result<Note> {
        match s.as_ref() {
            "A" => Ok(Note::A),
            "A#" | "Bb" => Ok(Note::ASharp),
            "B" => Ok(Note::B),
            "C" => Ok(Note::C),
            "C#" | "Db" => Ok(Note::CSharp),
            "D" => Ok(Note::D),
            "D#" | "Eb" => Ok(Note::DSharp),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "F#" | "Gb" => Ok(Note::FSharp),
            "G" => Ok(Note::G),
            "G#" | "Ab" => Ok(Note::GSharp),
            "" => bail!("note cannot be empty"),
            _ => bail!("unrecognized note: {}", s),
        }
    }

    pub fn next(&self) -> Note {
        match *self {
            Note::A => Note::ASharp,
            Note::ASharp => Note::B,
            Note::B => Note::C,
            Note::C => Note::CSharp,
            Note::CSharp => Note::D,
            Note::D => Note::DSharp,
            Note::DSharp => Note::E,
            Note::E => Note::F,
            Note::F => Note::FSharp,
            Note::FSharp => Note::G,
            Note::G => Note::GSharp,
            Note::GSharp => Note::A,
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B",
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_note_from_string() {
        let inputs = [
            String::from("A"),
            String::from("Ab"),
            String::from("C#"),
            String::from("F"),
        ];

        for input in &inputs {
            assert!(Note::try_from_string(input).is_ok())
        }

        let inputs = [
            String::from("Fb"),
            String::from("applesauce"),
            String::from("H#"),
        ];

        for input in &inputs {
            assert!(Note::try_from_string(input).is_err())
        }
    }
}