use std::io;

fn main() {
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("Expected a username !");
    
    println!("Bonjour {}, comment allez-vous ?", username);
}
