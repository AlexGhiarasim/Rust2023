fn add_chars_n(mut s: String, ch: char, n: i32) -> String {
    let mut i = 1;
    while i < n {
        i = i + 1;
        s.push(ch);
    }
    return s;
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
