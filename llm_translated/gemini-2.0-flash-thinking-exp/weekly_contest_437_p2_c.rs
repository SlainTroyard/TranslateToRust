use std::io;
use std::io::BufRead;

fn max_weight(pizzas: &[i32]) -> i64 {
    let pizzas_size = pizzas.len();
    let day = pizzas_size / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Create a mutable copy to sort
    let mut sorted_pizzas = pizzas.to_vec();
    sorted_pizzas.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    let mut total: i64 = 0;
    let mut index = 0;

    for _ in 0..odd {
        total += sorted_pizzas[index] as i64;
        index += 1;
    }

    index += 1; // Increment index once more after the first loop as in original C code

    for _ in 0..even {
        if index < pizzas_size { // Add check to prevent out of bounds access
            total += sorted_pizzas[index] as i64;
            index += 2;
        } else {
            break; // Break if index exceeds pizza size
        }
    }
    total
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut n_str = String::new();
    handle.read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input for n");

    let mut pizzas_str = String::new();
    handle.read_line(&mut pizzas_str).expect("Failed to read line");
    let pizzas: Vec<i32> = pizzas_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for pizza value"))
        .collect();

    println!("{}", max_weight(&pizzas));
}