fn maxim(a: i32, b: i32) -> Option<i32> {
    if a > b {
        Some(a)
    } else if b > a {
        Some(b)
    } else {
        None
    }
}

fn main() {
    let nr1 = 10;
    let nr2 = 20;

    match maxim(nr1, nr2) {
        Some(max) => println!("The maximum value is: {}", max),
        None => println!("The numbers are equal."),
    }
}
