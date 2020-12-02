/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let chars:Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
    
    if chars.len() <= 1 || chars.iter().any(|ch| !ch.is_numeric())  {
        return false
    }
    
    chars
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| {
            let digit = x.to_digit(10).unwrap_or_default();

            match i {
                i if i % 2 == 0 => digit,
                _ if digit * 2 > 9 => digit * 2 - 9,
                _ => digit * 2
            }
        })
        .sum::<u32>() % 10 == 0
}
