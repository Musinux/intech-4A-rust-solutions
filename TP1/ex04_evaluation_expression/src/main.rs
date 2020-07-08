use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    println!("Entrez x:");
    io::stdin()
        .read_line(&mut x)
        .expect("Veuillez entrer une valeur");
    
    let x: i32 = x.trim().parse().expect("Il fallait rentrer un nombre !");
    println!("Entrez y:");
    io::stdin()
        .read_line(&mut y)
        .expect("Veuillez entrer une valeur");
    
    
    let y: i32 = y.trim().parse().expect("Il fallait rentrer un nombre !");

    let sum = x + y;
    println!("{} + {} = {}", x, y, sum);
}
