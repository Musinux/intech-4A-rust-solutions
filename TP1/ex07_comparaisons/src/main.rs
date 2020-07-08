use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    let mut y = String::new();

    println!("Entrez un premier nombre:");
    io::stdin().read_line(&mut y)
        .expect("Une erreur s'est produite");

    let y: i32 = y.trim().parse()
        .expect("Vous n'avez pas entré un nombre.");

    println!("Entrez un second nombre:");
    let mut z = String::new();

    io::stdin().read_line(&mut z)
        .expect("Une erreur s'est produite");

    let z: i32 = z.trim().parse()
        .expect("Vous n'avez pas entré un nombre.");
    
    match y.cmp(&z) {
        Ordering::Equal => println!("{} == {}", y, z),
        Ordering::Less => println!("{} < {}", y, z),
        Ordering::Greater => println!("{} > {}", y, z),
    }
}
