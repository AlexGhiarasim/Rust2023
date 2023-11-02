use std::io;

fn main() {
    let mut text = String::new();

    println!("Write a text:");
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input");

    let result = rot13(&text);

    match result {
        Ok(encrypted) => println!("ROT13: {}", encrypted),
        Err(error) => eprintln!("Error: {}", error),
    }
}
fn rot13(text: &str) -> Result<String, String> {
    let mut result = String::new();

    for c in text.chars() {
        match c {
            'a'..='z' => {
                let shifted = ((c as u8 - b'a' + 13) % 26) + b'a';
                result.push(char::from(shifted));
            }
            'A'..='Z' => {
                let shifted = ((c as u8 - b'A' + 13) % 26) + b'A';
                result.push(char::from(shifted));
            }
            ' ' | '\n' | '\t' | '\r' => result.push(c),
            _ => {
                return Err(format!(
                    "Error: Non-ASCII character is encountered: '{}'",
                    c
                ))
            }
        }
    }
    Ok(result)
}

//input: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
//output: NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm
