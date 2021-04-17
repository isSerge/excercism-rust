pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let power = str.len() as u32;

    str.chars()
        .map(|c| c.to_digit(10).unwrap().pow(power))
        .sum::<u32>()
        .eq(&num)
}
