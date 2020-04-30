use std::collections::HashMap;

static VALID: [char; 4] = ['A', 'C', 'G', 'T'];

fn validate(c: char) -> Result<char, char> {
    if VALID.iter().any(|x| *x == c) {
        Ok(c)
    } else {
        Err(c)
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    validate(nucleotide).and_then(|nucleotide| {
        dna.chars()
            .map(validate)
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.into_iter().filter(|x| *x == nucleotide).count())
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    VALID
        .iter()
        .map(|c| count(*c, dna).map(|size| (*c, size)))
        .collect()
}
