struct Classroom {
    name: String,
    students: Vec<String>,
}

impl Classroom {
    fn add_student(&mut self, student: &str) {
        self.students.push(student.to_string());
    }
}

fn main() {
    let mut classroom = Classroom {
        name: "ABC123".to_string(),
        students: Vec::new(),
    };
    classroom.add_student("Billy Bob");
    println!("Students: {:?}", classroom.students);
}
