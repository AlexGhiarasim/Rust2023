use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let abbreviations = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    let input_text = fs::read_to_string("fisier.txt")?;
    let result = replace(&input_text, &abbreviations);
    println!("{}", result);
    Ok(())
}
fn replace(input: &str, abbreviations: &[(&str, &str)]) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut result = String::new();
    for word in words {
        let mut replaced = false;
        for &(abbrev, full) in abbreviations {
            if word == abbrev {
                result.push_str(full);
                result.push(' ');
                replaced = true;
                break;
            }
        }
        if !replaced {
            result.push_str(word);
            result.push(' ');
        }
    }
    if !result.is_empty() {
        result.pop();
    }
    result
}
