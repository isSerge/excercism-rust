#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<'a, T: PartialEq + Clone + Ord> CustomSet<T> {
    pub fn new(input: &'a[T]) -> Self {
        let mut items = input.to_vec();

        items.sort();
        items.dedup();

        Self { items }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.items.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.items.push(element);
            self.items.sort();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.is_empty() || other.items.windows(self.items.len()).any(|w| w.to_vec() == self.items)
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let items:Vec<_> = self.items
            .clone()
            .into_iter()
            .filter(|item| other.items.contains(item))
            .collect();

        Self::new(items.as_slice())
    }

    pub fn difference(&self, other: &Self) -> Self {
        let items:Vec<_> = self.items
            .clone()
            .into_iter()
            .filter(|item| !other.items.contains(item))
            .collect();

        Self::new(items.as_slice())
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut xs = self.items.clone();
        let mut ys = other.items.clone();

        xs.append(&mut ys);
        
        Self::new(xs.as_slice())
    }
}
