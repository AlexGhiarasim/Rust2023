#[derive(Debug)]
enum Error {
    OverflowErrorType,
}

fn check_addition(n1: u32, n2: u32) -> Result<u32, Error> {
    match n1.checked_add(n2) {
        Some(result) => Ok(result),
        None => Err(Error::OverflowErrorType),
    }
}

fn check_multiply(n1: u32, n2: u32) -> Result<u32, Error> {
    match n1.checked_mul(n2) {
        Some(result) => Ok(result),
        None => Err(Error::OverflowErrorType),
    }
}
fn usage(n1: u32, n2: u32) -> Result<u32, Error> {
    let sum = check_addition(n1, n2)?;
    let product = check_multiply(sum, n1)?;
    Ok(product)
}
fn main() {
    match usage(500, 60000) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
