use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/fisier.txt")?;
    let mut n1 = 0;
    let mut n2 = 0;
    let mut str1 = String::new();
    let mut str2 = String::new();

    for line in s.lines() {
        let mut nrch = 0;
        let mut nrbytes = 0;

        for ch in line.chars() {
            nrch += 1;
            if ch.is_alphanumeric() && !ch.is_ascii() {
                nrbytes += 2;
            } else if ch.is_ascii_alphanumeric() {
                nrbytes += 1;
            } else if !ch.is_alphanumeric() {
                nrbytes += 4;
            }
        }

        if n1 < nrbytes {
            n1 = nrbytes;
            str1 = line.to_string();
        }

        if n2 < nrch {
            n2 = nrch;
            str2 = line.to_string();
        }
    }

    println!("Longest line considering the number of bytes: {}", str1);
    println!("Longest line considering the number of characters: {}",str2);
    Ok(())
}
