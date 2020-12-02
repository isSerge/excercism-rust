extern crate luhn;
use luhn::is_valid;

pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        is_valid(&self.0)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
