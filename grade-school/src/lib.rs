use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct School {
    grade_students: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School { grade_students: BTreeMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grade_students
            .entry(grade)
            .or_insert(BTreeSet::new())
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grade_students.keys().map(|k| *k).collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // thpe internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.grade_students.get(&grade) {
            Some(v) => Some(v.iter().cloned().collect::<Vec<String>>()),
            None => None,
        }
    }
}
