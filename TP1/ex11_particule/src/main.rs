fn charge_totale (charge1: i32, charge2: i32) -> i32 {
    match (charge1, charge2) {
        (-1, 1) | (1, -1) => 0,
        (-1, -1) => -1,
        (1, 1) => 1,
        _ => panic!("Les charges n'ont pas des valeurs cohérentes"),
    }
}

fn main() {
    println!("Hello, world!");
    println!("-1 -1: {}", charge_totale(-1, -1));
    println!(" 1  1: {}", charge_totale(1, 1));
    println!("-1  1: {}", charge_totale(-1, 1));
    println!(" 1 -1: {}", charge_totale(1, -1));
}
