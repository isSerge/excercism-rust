/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_ascii_uppercase().chars().fold(0, |score, x| match x {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => score + 1,
        'D' | 'G' => score + 2,
        'B' | 'C' | 'M' | 'P' => score + 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => score + 4,
        'K' => score + 5,
        'J' | 'X' => score + 8,
        'Q' | 'Z' => score + 10,
        _ => score,
    })
}
