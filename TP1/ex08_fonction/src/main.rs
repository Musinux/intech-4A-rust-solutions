use std::io;
use std::cmp::Ordering;

fn enter_number() -> i32 {
    let mut y = String::new();

    println!("Entrez un nombre:");
    io::stdin().read_line(&mut y)
        .expect("Une erreur s'est produite");

    let y: i32 = y.trim().parse()
        .expect("Vous n'avez pas entré un nombre.");
    y
}

fn main() {
    let y = enter_number();
    let z = enter_number();
    
    match y.cmp(&z) {
        Ordering::Equal => println!("{} == {}", y, z),
        Ordering::Less => println!("{} < {}", y, z),
        Ordering::Greater => println!("{} > {}", y, z),
    }
}
