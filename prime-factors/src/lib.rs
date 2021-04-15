pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut current_factor = 2;

    while n > 1 {
        while n % current_factor == 0 {
            n /= current_factor;
            factors.push(current_factor);
        }

        current_factor += 1;
    }

    factors
}