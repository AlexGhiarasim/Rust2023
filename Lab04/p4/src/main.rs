use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let hosts_path = "C:\\Windows\\System32\\drivers\\etc\\hosts"; //path

    let s1 = fs::read_to_string(hosts_path)?;
    let mut result = String::new();
    let mut previous_word = "sd";

    for line in s1.lines() {
        if !line.starts_with("#") {
            let words: Vec<&str> = line.split_whitespace().collect();
            for (index, word) in words.iter().enumerate() {
                if index % 2 == 1 {
                    result.push_str(word);
                    result.push_str(" => ");
                    result.push_str(previous_word);
                    result.push_str("\n");
                } else {
                    previous_word = word;
                }
            }
        }
    }
    println!("{}", result);
    Ok(())
}
