use itertools::Itertools;
use regex::Regex;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .group_by(|&x| x)
        .into_iter()
        .map(|(ch, group)| match group.count() {
            1 => ch.to_string(),
            num =>  format!("{}{}", num, ch),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    let regex = Regex::new(r"\d*\D").unwrap();

    regex
        .captures_iter(source)
        .map(|x| x.get(0).map_or("", |y| y.as_str()))
        .map(|x| {
            let (num, ch): (String, String)  = x.chars().partition(|c| c.is_numeric());
            ch.repeat(num.parse().unwrap_or(1))
        })
        .collect()
}
