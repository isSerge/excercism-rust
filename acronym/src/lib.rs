pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|x| x == ' ' || x == '-')
        .filter(|w| !w.is_empty())
        .map(|x| {
            let alphabetic_chars:Vec<_> = x.chars().filter(|c| c.is_alphabetic()).collect();
            let uppercase_chars:Vec<_> = alphabetic_chars.iter().filter(|c| c.is_uppercase()).collect();

            if uppercase_chars.len() > 1 && uppercase_chars.len() != alphabetic_chars.len() {
                uppercase_chars.into_iter().collect()
            } else {
                alphabetic_chars.get(0).unwrap().to_string()
            }
        })
        .collect::<String>()
        .to_uppercase()
}