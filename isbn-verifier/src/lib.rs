pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits = isbn.chars()
        .filter_map(|s| s.to_digit(10))
        .collect::<Vec<u32>>();

    match digits.len() {
        9 if isbn.ends_with('X') => (calc_isbn_formula(digits) + 10) % 11 == 0,
        10 => calc_isbn_formula(digits) % 11 == 0,
        _ => false,
    }
}

fn calc_isbn_formula(digits: Vec<u32>) -> u32 {
    digits
        .iter()
        .enumerate()
        .map(|(i, n)| n * (10 - (i as u32)))
        .sum::<u32>()
}