use std::cmp::Ordering;

fn charge_somme (charges: Vec<i32>) -> i32 {
    let mut result = 0;
    for c in charges.iter() {
        result += c;
    }
    // alternative
    /*
    result = charges.iter().sum();
    */
    match result.cmp(&0) {
        Ordering::Equal => 0,
        Ordering::Less => -1,
        Ordering::Greater => 1,
    }
}

fn main() {
    let charges = vec![1, -1, 1, -1, -1, -1, -1];
    println!("resultat: {}", charge_somme(charges));
}
