use std::collections::HashMap;

pub struct School {
    data: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student_list = self.data.entry(grade).or_insert(vec![]);
        (*student_list).push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.data.keys().copied().collect::<Vec<u32>>();
        grades.sort();
        grades.dedup();
        grades
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.data.get(&grade) {
            None => None,
            Some(v) => {
                let mut res = (*v.to_owned()).to_vec();
                res.sort();
                Some(res)
            }
        }
    }
}
