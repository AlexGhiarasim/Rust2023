use std::fs;

#[derive(Debug)]
struct Student {
    #[allow(dead_code)] //to cover the warning, I hope that it's ok.
    name: String,
    #[allow(dead_code)]
    phone: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "file.txt"; //file
    let input_text = fs::read_to_string(path)?;

    let mut students: Vec<Student> = Vec::new();
    for line in input_text.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let name = String::from(parts[0]);
            let phone = String::from(parts[1]);
            let age: u32 = parts[2].parse()?;
            let student = Student { name, phone, age };
            students.push(student);
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    let oldest_student = students.iter().max_by_key(|s| s.age);
    let youngest_student = students.iter().min_by_key(|s| s.age);
    //prints
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
    Ok(()) //success
}
