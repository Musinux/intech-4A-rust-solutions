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
    let a = enter_number();
    let b = enter_number();
    
    // possibilité n°1:
    match a.cmp(&b) {
        Ordering::Less => {
            panic!("{} < {}, stop.", a, b);
        },
        _ => {},
    }
    // possibilité n°2:
    if let Ordering::Less = a.cmp(&b) {
        panic!("{} < {}, stop.", a, b);
    }

    let y = enter_number();
    if a <= y && y <= b {
        println!("{} est dans l'intervalle [{}, {}]", y, a, b);
    } else {
        println!("{} n'est pas dans l'intervalle [{}, {}]", y, a, b);
    }
}
