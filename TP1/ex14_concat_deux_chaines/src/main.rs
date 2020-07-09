use std::io;
/**
 * Cette fonction prend des paramètres immuables car on n'effectue qu'une copie de la chaîne
 * Ce sont des références car cette fonction ne fait qu'emprunter, elle ne déplace pas l'appartenance
 */
fn concat(str1: &str, str2: &str) -> String {
    let mut str3 = String::from(str1); // on duplique
    str3.push_str(str2);               // on concatène
    str3                               // on retourne
}

fn main() {
    // on crée deux chaînes dynamiques
    let mut str1 = String::new();
    let mut str2 = String::new();
    println!("Enter some text:");
    io::stdin().read_line(&mut str1).expect("Expected text !");
    
    println!("Enter some other text:");
    io::stdin().read_line(&mut str2).expect("Expected text !");

    // ne retire que le dernier caractère,
    // sous windows deux caractères sont à retirer potentiellement \r\n
    str1.pop();
    str2.pop();

    let str3 = concat(&str1, &str2);
    println!("{}", str3);
}
