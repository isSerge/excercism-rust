#[derive(Debug, PartialEq)]
pub struct Dna(Vec<char>);

#[derive(Debug, PartialEq)]
pub struct Rna(Vec<char>);


const ALLOWED_DNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
const ALLOWED_RNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'U'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let invalid_nucleotide = dna.chars().find(|ch| !ALLOWED_DNA_NUCLEOTIDES.contains(&ch));

        match invalid_nucleotide {
            None => Ok(Dna(dna.chars().collect())),
            Some(ch) => Err(dna.chars().position(|c| c == ch).unwrap()),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.iter().map(|n| {
            match n {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                n => *n,
            }
        }).collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let invalid_nucleotide = rna.chars().find(|ch| !ALLOWED_RNA_NUCLEOTIDES.contains(&ch));

        match invalid_nucleotide {
            None => Ok(Rna(rna.chars().collect())),
            Some(ch) => Err(rna.chars().position(|c| c == ch).unwrap()),
        }
    }
}
