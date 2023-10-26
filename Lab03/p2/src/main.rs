
fn check_addition(n1: u32, n2: u32) -> u32 {
    match n1.checked_add(n2) {
        Some(result) => result,
        None => panic!("Addition between {} and {} doesn't fit in u32!", n1, n2),
    }
}

fn check_multiply(n1: u32, n2: u32) -> u32 {
    match n1.checked_mul(n2) {
        Some(result) => result,
        None => panic!(
            "Multiplication between {} and {} doesn't fit in u32!",
            n1, n2
        ),
    }
}
fn main() {
    println!("{}", check_addition(10, 34));
    println!("{}", check_multiply(15, 23));
}
