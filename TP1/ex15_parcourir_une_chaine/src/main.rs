fn print_string(s: &str) {
    for (i, c) in s.chars().enumerate() {
        println!("{}: {}", i, c);
    }
}
fn print_string_byte(s: &str) {
    for (i, c) in s.bytes().enumerate() {
        println!("{}: {}", i, c);
    }
}

fn main() {
    println!("Hello, world!");

    let s = "Bonjour";
    print_string(s);
    print_string_byte(s);
}
