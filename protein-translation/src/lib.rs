use std::str;
use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(str::from_utf8)
            .take_while(|x| !["UAA", "UAG", "UGA"].contains(&x.unwrap()))
            .fold(Some(Vec::new()),|acc, chunk| {
                let chunk = chunk.unwrap();

                if chunk.len() != 3 {
                    return None
                }

                match self.name_for(chunk) {
                    Some(protein) => {
                        let mut acc = acc.unwrap();
                        acc.push(protein);
                        Some(acc)
                    },
                    None => None,
                }
            })
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.into_iter().collect()
    }
}
