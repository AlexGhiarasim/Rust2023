use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Student {
    #[allow(dead_code)] //it must write these sections
    name: String,
    #[allow(dead_code)]
    phone: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "file.txt"; //fisier
    let input_text = fs::read_to_string(path)?;
    let mut students: Vec<Student> = Vec::new();

    for line in input_text.lines() {
        let student: Student = serde_json::from_str(line)?;
        students.push(student);
    }

    let oldest_student = students.iter().max_by_key(|s| s.age);
    let youngest_student = students.iter().min_by_key(|s| s.age);

    if let Some(oldest) = oldest_student {
        println!("Oldest student: {:?}", oldest);
    } else {
        println!("No students found");
    }

    if let Some(youngest) = youngest_student {
        println!("Youngest student: {:?}", youngest);
    } else {
        println!("No students found");
    }

    Ok(())
}
