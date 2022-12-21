fn main() {
    struct Student {
        name: String,
        age: u8,
        grade: char,
    }

    let student = Student {
        name: String::from("John"),
        age: 18,
        grade: 'A',
    };

    println!("Name: {}", student.name);
}
// This is a example of struct
