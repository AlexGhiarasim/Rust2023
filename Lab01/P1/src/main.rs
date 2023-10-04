fn prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n > 2 && n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    loop {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
        i = i + 1;
    }
    return true;
}

fn coprime(mut n1: i32, mut n2: i32) -> bool {
    let mut r: i32;
    while n2 != 0 {
        r = n1 % n2;
        n1 = n2;
        n2 = r;
    }
    if n1 == 1 {
        return true;
    }
    return false;
}

fn singing() {
    let mut nrbeers = 99;
    while nrbeers > 1 {
        println!("{} bottles of beer on the wall,", nrbeers);
        println!("{} bottles of beer.", nrbeers);
        println!("Take one down, pass it around,");
        nrbeers = nrbeers - 1;
        println!("{} bottles of beer on the wall.", nrbeers);
        println!("\n");
    }
    println!("{} bottle of beer on the wall,", nrbeers);
    println!("{} bottle of beer.", nrbeers);
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
    println!("\n");

    println!("No bottles of beer on the wall,");
    println!("No bottles of beer.");
    println!("Go to the store, buy some more,");
    println!("99 bottles of beer on the wall.");
}

fn main() {
    //P1
    for i in 0..100 {
        if prime(i) == true {
            println!("{} is prime", i);
        }
    }
    println!("\n");

    //P2
    for i in 0..100 {
        for j in 0..100 {
            if coprime(i, j) == true {
                println!("{} and {} are coprimes!", i, j);
            }
        }
    }
    println!("\n");

    //P3
    singing();
}
