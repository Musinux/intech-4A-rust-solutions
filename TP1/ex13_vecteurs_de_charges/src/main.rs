fn charge_totale (charge1: i32, charge2: i32) -> i32 {
    match (charge1, charge2) {
        (-1, 1) | (1, -1) => 0,
        (-1, -1) => -1,
        (1, 1) => 1,
        _ => panic!("Les charges n'ont pas des valeurs cohérentes"),
    }
}

fn charge_somme (c1: Vec<i32>, c2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    
    for i in 0..10 {
        result.push(charge_totale(c1[i], c2[i]));
    }
    // alternative 1
    /*
    for (i, c) in c1.iter().enumerate() {
        result.push(charge_totale(*c, c2[i]));
    }
    */
    // alternative 2
    /*
    for (a, b) in c1.iter().zip(c2) {
        result.push(charge_totale(*a, b))
    }
    */
    // alternative 3
    /*
    result = c1.iter()
        .zip(c2)
        .map(|(a, b)| charge_totale(*a, b))
        .collect();
    */
    result
}

fn main() {
    let c1 = vec![1, -1, 1, -1, -1, -1, -1];
    let c2 = vec![1, -1, 1, -1, -1, -1, 1];
    println!("resultat: {:?}", charge_somme(c1, c2));
}
