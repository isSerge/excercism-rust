#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        (x, y) if x == y => Comparison::Equal, 
        (x, y) if x.len() > y.len() && is_subset_of(y, x) => Comparison::Superlist,
        (x, y) if x.len() < y.len() && is_subset_of(x, y) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}

pub fn is_subset_of<T: PartialEq>(short: &[T], long: &[T]) -> bool {
    short.is_empty() || long.windows(short.len()).any(|w| w == short)
}
