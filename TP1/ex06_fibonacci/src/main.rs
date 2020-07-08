fn main() {
    let mut t1 = 0;
    let mut t2 = 1;
    println!("{}", t1);
    for i in 0..10 {
        if i % 2 == 0 {
            println!("{}", t2);
            t1 += t2;
        } else {
            println!("{}", t1);
            t2 += t1;
        }
    }
}
