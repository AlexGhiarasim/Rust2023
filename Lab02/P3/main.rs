fn add_space(s: &mut String, n: i32) {
    let mut i = 1;
    while i <= n {
        s.push(' ');
        i = i + 1;
    }
}
fn add_str(s: &mut String, str: &str) {
    s.push_str(str);
}
fn add_integer(s: &mut String, mut nr: i64) {
    let mut nrdigits = 0;
    let mut ogl = 0;
    while nr != 0 {
        ogl = ogl * 10 + nr % 10;
        nr = nr / 10;
    }
    nr = ogl;
    while nr != 0 {
        nrdigits = nrdigits + 1;
        s.push((((nr % 10) as u8) + 48) as char);
        nr = nr / 10;
        if nrdigits == 3 && nr % 100 > 9 {
            s.push('_');
            nrdigits = 0;
        }
    }
}
fn add_float(s: &mut String, mut nr: f32) {
    let fractional: f32 = nr.floor();
    add_integer(s, fractional as i64);
    s.push('.');
    while nr.floor() != nr {
        nr = nr * 10.0;
        s.push(((((nr.floor()) as u8) % 10) + 48) as char)
    }
}
fn main() {
    let mut mystring: String = String::from("");
    add_space(&mut mystring, 40);
    add_str(&mut mystring, "I");
    add_space(&mut mystring, 1);
    add_str(&mut mystring, "💚");
    add_space(&mut mystring, 43);
    add_str(&mut mystring, "\n");
    add_space(&mut mystring, 40);
    add_str(&mut mystring, "RUST.");
    add_space(&mut mystring, 42);
    add_str(&mut mystring, "\n");
    add_space(&mut mystring, 86);
    add_str(&mut mystring, "\n");
    add_space(&mut mystring, 4);
    add_str(&mut mystring, "Most");
    add_space(&mut mystring, 12);
    add_str(&mut mystring, "crate");
    add_space(&mut mystring, 6);
    add_integer(&mut mystring, 306437968);
    add_space(&mut mystring, 11);
    add_str(&mut mystring, "and");
    add_space(&mut mystring, 5);
    add_str(&mut mystring, "lastest");
    add_space(&mut mystring, 9);
    add_str(&mut mystring, "is");
    add_space(&mut mystring, 9);
    add_str(&mut mystring, "\n");
    add_space(&mut mystring, 9);
    add_str(&mut mystring, "downloaded");
    add_space(&mut mystring, 8);
    add_str(&mut mystring, "has");
    add_space(&mut mystring, 13);
    add_str(&mut mystring, "downloads");
    add_space(&mut mystring, 5);
    add_str(&mut mystring, "the");
    add_space(&mut mystring, 9);
    add_str(&mut mystring, "version");
    add_space(&mut mystring, 7);
    add_float(&mut mystring, 2.5038);
    add_str(&mut mystring, ".");
    add_space(&mut mystring, 86);
    println!("{}", mystring);
}
