use std::collections::HashMap;

fn is_nucleotide(n: char) -> bool {
    n == 'A' || n == 'C' || n == 'G' || n == 'T'
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => {
            let mut count = 0;
            for c in dna.chars() {
                if !is_nucleotide(c) {
                    return Err(c);
                }
                if c == nucleotide {
                    count += 1;
                }
            }
            return Ok(count);
        }
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = std::collections::HashMap::<char, usize>::new();

    map.insert('A', 0);
    map.insert('T', 0);
    map.insert('C', 0);
    map.insert('G', 0);

    for c in dna.chars() {
        if !is_nucleotide(c) {
            return Err(c);
        }
        let entry = map.entry(c).or_insert(0);
        *entry += 1;
    }
    Ok(map)
}
