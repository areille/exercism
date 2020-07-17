#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

fn is_valid_char_dna(c: char) -> bool {
    c == 'A' || c == 'C' || c == 'G' || c == 'T'
}

fn is_valid_char_rna(c: char) -> bool {
    c == 'A' || c == 'C' || c == 'G' || c == 'U'
}

fn transcript_char(c: char) -> char {
    match c {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => unreachable!(),
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for i in 0..dna.chars().count() {
            if !is_valid_char_dna(dna.chars().nth(i).unwrap()) {
                return Err(i);
            }
        }
        Ok(DNA(dna.to_string()))
    }

    pub fn into_rna(self) -> RNA {
        RNA(self.0.chars().map(transcript_char).collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for i in 0..rna.chars().count() {
            if !is_valid_char_rna(rna.chars().nth(i).unwrap()) {
                return Err(i);
            }
        }
        Ok(RNA(rna.to_string()))
    }
}
