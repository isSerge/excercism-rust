use std::collections::HashSet;

fn sort_chars (x: &str) -> Vec<char> {
    let mut x = x.chars().collect::<Vec<char>>();
    x.sort();
    x
}

fn is_anagram_of (x: &str, y: &str) -> bool {
    let x = x.to_lowercase();
    let y = y.to_lowercase();

    if x == y || x.len() != y.len() {
        return false
    }

    sort_chars(&x) == sort_chars(&y)
} 

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    possible_anagrams.iter().fold(HashSet::new(), |mut set, x| {
        if is_anagram_of(x, word) {
            set.insert(x);
            set
        } else {
            set
        }
    })
}
