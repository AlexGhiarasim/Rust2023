#[derive(Debug)]
enum CharError {
    NotALetter,
    NotAPrintableChar,
    NotASCIIDigit,
    NotHexDigit,
}
fn to_uppercase(c: char) -> Result<char, CharError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(CharError::NotALetter)
    }
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(CharError::NotALetter)
    }
}
fn print_char(c: char) -> Result<char, CharError> {
    if c.is_ascii_graphic() {
        println!("Character {} is printable!", c);
        Ok(c)
    } else {
        Err(CharError::NotAPrintableChar)
    }
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if c.is_ascii() && c.is_digit(10) {
        Ok(c.to_digit(10).unwrap())
    } else {
        Err(CharError::NotASCIIDigit)
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if c.is_ascii() && c.is_digit(16) {
        Ok(c.to_digit(16).unwrap())
    } else {
        Err(CharError::NotHexDigit)
    }
}

fn print_error(error: CharError) {
    match error {
        CharError::NotALetter => {
            println!("Error: Character is not a letter.");
        }
        CharError::NotAPrintableChar => {
            println!("Error: Character is no printable.");
        }
        CharError::NotASCIIDigit => {
            println!("Error: Character is not an ASCII digit.");
        }
        CharError::NotHexDigit => {
            println!("Error: Character is not a hexadecimal-digit.");
        }
    }
}
fn main() {
    match to_uppercase('d') {
        Ok(result) => println!("Uppercase: {}", result),
        Err(error) => print_error(error),
    }
    match to_lowercase('A') {
        Ok(result) => println!("Lowercase: {}", result),
        Err(error) => print_error(error),
    }
    match print_char('X') {
        Ok(_) => {}
        Err(error) => print_error(error),
    }
    match char_to_number('7') {
        Ok(result) => println!("Number: {}", result),
        Err(error) => print_error(error),
    }
    match char_to_number_hex('C') {
        Ok(result) => println!("Hexadecimal Number: {}", result),
        Err(error) => print_error(error),
    }
    match to_uppercase('&') {
        Ok(result) => println!("Uppercase: {}", result),
        Err(error) => print_error(error),
    }
    match char_to_number_hex('K') {
        Ok(result) => println!("Hexadecimal Number: {}", result),
        Err(error) => print_error(error),
    }
    match char_to_number('}') {
        Ok(result) => println!("Number: {}", result),
        Err(error) => print_error(error),
    }
}
