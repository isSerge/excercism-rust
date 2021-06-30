use std::ops::{Add, Sub};

pub struct Triangle<T>([T; 3]);

impl<T: Default + Copy + PartialEq + PartialOrd + Add<Output=T> + Sub<Output=T>>
    Triangle<T>
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let zero = T::default();
        let [a, b, c] = sides;

        if a == zero || b == zero || c == zero {
            return None;
        }

        let mut sorted = sides;
        
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let [a, b, c] = sorted;

        if c - (a + b) > T::default() {
            None
        } else {
            Some(Triangle(sides))
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.0;

        a == b && b == c
    }

    pub fn is_scalene(&self) -> bool {
        let [a, b, c] = self.0;

        a != b && a != c && b != c
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.0;

        a == b || a == c || b == c
    }
}
