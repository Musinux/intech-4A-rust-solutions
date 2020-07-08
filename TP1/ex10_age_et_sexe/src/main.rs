use std::io;

const MAX_POINTS: u32 = 100_000;

fn enter_number(to_enter: &str) -> i32 {
    let mut y = String::new();

    println!("Entrez {}:", to_enter);
    io::stdin().read_line(&mut y)
        .expect("Une erreur s'est produite");

    let y: i32 = y.trim().parse()
        .expect("Vous n'avez pas entré un nombre.");
    y
}

fn main() {
    let mut name = String::new();

    println!("Entrez votre nom:");
    io::stdin().read_line(&mut name)
        .expect("Une erreur s'est produite");

    let year = enter_number("votre année de naissance");

    let mut sex = String::new();

    println!("Entrez votre sexe (H/F):");
    io::stdin().read_line(&mut sex)
        .expect("Une erreur s'est produite");
    
    print!("Bonjour {}, ", name.trim());

    let more_than_sixteen = if year <= 2004 {
        print!("vous avez ");
        true
    } else {
        print!("tu as ");
        false
    };

    print!("{} ans et ", 2020 - year);

    if more_than_sixteen == true {
        match sex.trim() {
            "H" => println!("vous êtes un homme"),
            "F" => println!("vous êtes une femme"),
            _ => panic!("Erreur"),
        }
    } else {
        match sex.trim() {
            "H" => println!("tu es un garçon"),
            "F" => println!("tu es une fille"),
            _ => panic!("Erreur"),
        }
    }
}
