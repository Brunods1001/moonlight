use std::char;

use crate::urls;

#[derive(Clone)]
pub struct AminoAcid {
    pub id: i32,
    pub amino_acid: char,
    pub position: i32,
}

#[derive(Clone)]
pub struct Sequence {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub sequence: String,
    pub amino_acids: Vec<AminoAcid>,
}

impl Sequence {
    pub fn new(id: i32, name: String, description: String, sequence: String) -> Sequence {
        let amino_acids = sequence.chars().enumerate().map(|(i, c)| AminoAcid {
            id: i as i32,
            amino_acid: c,
            position: (i as i32 + 1),
        }).collect();
        Sequence {
            id,
            name,
            description,
            sequence,
            amino_acids,
        }
    }
}

pub struct SequenceUrl {
    pub sequence: Sequence,
    pub url: String,
}

impl SequenceUrl {
    pub fn new(sequence: Sequence) -> SequenceUrl {
        let url = format!(
            "{}/{}",
            urls::sequences(),
            sequence.id,
        );
        SequenceUrl {
            sequence,
            url,
        }
    }
}
