fn prime(x: u16) -> bool {
    if x == 0 || x == 1 {
        return false;
    }
    if x % 2 == 0 || x % 3 == 0 {
        return false;
    }
    let mut i: u32 = 3;
    while i * i <= x as u32 {
        if x % i as u16 == 0 {
            return false;
        }
        i = i + 1;
    }
    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut next_number = x;
    while next_number < u16::MAX {
        if prime(next_number) == true {
            return Some(next_number);
        }
        next_number = next_number + 1;
    }
    return None;
}
fn main() {
    let mut number = 63000;
    loop {
        match next_prime(number) {
            Some(prime) => {
                println!("Number {prime} is prime");
                number = prime + 1;
            }
            None => {
                println!("65536 doesn't fit in u16! The Loop has finished!");
                break;
            }
        }
    }
}
