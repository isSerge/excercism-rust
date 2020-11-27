use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();

    candidate
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .all(|c| set.insert(c))
}