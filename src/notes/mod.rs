use errors::*;

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
    pub fn try_from_string<S: Into<String>>(s: S) -> Result<Note> {
        let s = s.into();

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
            _ => bail!("Unrecognized note: {}", s),
        }
    }
}