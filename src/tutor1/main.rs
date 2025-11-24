struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: String) -> Self {
        Student {
            name,
            grades: Vec::new(),
        }
    }

    fn add_grade(&mut self, grade: f64) {
        if grade >= 0.0 && grade <= 100.0 {
            self.grades.push(grade);
        }
    }

    fn average(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            let sum: f64 = self.grades.iter().sum();
            sum / self.grades.len() as f64
        }
    }

    fn get_letter_grade(&self) -> Option<char> {
        let avg: f64 = self.average();
        if avg >= 90.0 {
            Some('A')
        } else if avg >= 80.0 {
            Some('B')
        } else if avg >= 70.0 {
            Some('C')
        } else if avg >= 60.0 {
            Some('D')
        } else if avg >= 0.0 {
            Some('F')
        } else {
            None
        }
    }
}

fn main() {
    let mut student = Student::new(String::from("Alice"));

    student.add_grade(85.0);
    student.add_grade(92.0);
    student.add_grade(78.0);

    println!("Student: {}", student.name);
    println!("Average Grade: {:.2}", student.average());
    println!("Letter Grade: {:?}", student.get_letter_grade());
}
