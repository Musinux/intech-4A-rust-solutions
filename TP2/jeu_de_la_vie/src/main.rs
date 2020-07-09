fn print_state(cellules: &Vec<Vec<i32>>) {
    println!("");
    for line in cellules {
        print!("|");
        for &cell in line {
            print!(" {} |", if cell == 0 { ' ' } else { 'O' });
        }
        println!("");
    }
}

fn sum_line (line: &Vec<i32>, center: usize) -> i32 {
    let mut curr = 0;
    
    // middle
    curr += line[center];
    
    // left
    if center > 0 {
        curr += line[center - 1];
    }
    // right
    if center + 1 < line.len() {
        curr += line[center + 1];
    }
    
    curr
}

fn sum_neighbours(cellules: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut curr = 0;
    if i > 0 {
        curr += sum_line(&cellules[i - 1], j);
    }

    if i + 1 < cellules.len() {
        curr += sum_line(&cellules[i + 1], j);
    }
    
    if j > 0 {
        curr += cellules[i][j - 1];
    }
    if j + 1 < cellules[i].len() {
        curr += cellules[i][j + 1];
    }
    curr
}

fn dead_or_alive(cell: i32, neighbours: i32) -> i32 {
    if neighbours == 3 || (neighbours == 2 && cell == 1) {
        1
    }
    else {
        0
    }
}

fn next_state(cellules: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_state = Vec::new();
    let mut i = 0;
    let mut j;
    for line in cellules {
        j = 0;
        let mut new_line = Vec::new();
        for &cell in line {
            let neighbours = sum_neighbours(&cellules, i, j);
 
            new_line.push(dead_or_alive(cell, neighbours));
            j += 1;
        }
        new_state.push(new_line);
        i += 1;
    }
    new_state
}

fn same_state(new_cellules: &Vec<Vec<i32>>, cellules: &Vec<Vec<i32>>) -> bool {
    for i in 0..new_cellules.len() {
        for j in 0..new_cellules.len() {
            if new_cellules[i][j] != cellules[i][j] {
                return false
            }
        }
    }
    true
}

fn main() {
    let mut cellules = vec![
        vec![1, 0, 1, 0, 1],
        vec![0, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![1, 1, 0, 0, 1],
        vec![0, 0, 0, 0, 1]
    ];

    loop {
        print_state(&cellules);
        let new_cellules = next_state(&cellules);

        if same_state(&new_cellules, &cellules) {
            break;
        }
        cellules = new_cellules;
    }
}
