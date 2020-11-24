use std::collections::HashMap;

const ALLOWED_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !ALLOWED_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide)
    }

    dna.chars().fold(Ok(0), |acc, ch| {
        if !ALLOWED_NUCLEOTIDES.contains(&ch) {
            Err(ch)
        } else if ch == nucleotide {
            Ok(acc.unwrap_or(0) + 1)
        } else {
            acc
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
        let mut map = HashMap::new();

        for nucleotide in ALLOWED_NUCLEOTIDES.iter() {
            map.insert(*nucleotide, count(*nucleotide, dna)?);
        }

        Ok(map)
}
