use std::collections::HashMap;

pub struct School(HashMap<u32, Vec<String>>);

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}

impl School {
    pub fn new() -> School {
        School(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let grade = self.0.entry(grade).or_default();
        grade.push(student.to_string());
        grade.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades:Vec<u32> = self.0.keys().cloned().collect();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.0.get(&grade).cloned()

    }
}
