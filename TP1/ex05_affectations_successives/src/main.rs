fn main() {
    let n: i32 = 2;

    for p2 in 0..6 {
        print!("{}:{} ", p2, n.pow(p2));
    }
}
