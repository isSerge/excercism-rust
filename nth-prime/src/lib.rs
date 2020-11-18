fn is_prime (n: u32) -> bool {
    !(2..n-1).any(|i| n % i == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|x| is_prime(*x)).nth(n as usize).unwrap()
}
