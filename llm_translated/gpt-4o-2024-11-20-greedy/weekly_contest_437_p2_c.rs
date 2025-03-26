use std::io::{self, BufRead};
use std::cmp::Reverse;

fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
    let pizzas_size = pizzas.len();
    let day = pizzas_size / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort pizzas in descending order
    pizzas.sort_by_key(|&x| Reverse(x));

    let mut total: i64 = 0;
    let mut index = 0;

    // Add the largest `odd` elements
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }

    // Skip one element
    index += 1;

    // Add the next `even` elements, skipping every other element
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }

    total
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of pizzas
    let n: usize = lines
        .next()
        .expect("Expected input for number of pizzas")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse number of pizzas");

    // Read the array of pizzas
    let mut pizzas: Vec<i32> = lines
        .next()
        .expect("Expected input for pizzas array")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse pizza weight"))
        .collect();

    // Ensure the input size matches the expected number of pizzas
    assert_eq!(pizzas.len(), n, "Input size does not match the number of pizzas");

    // Calculate the maximum weight
    let result = max_weight(&mut pizzas);

    // Print the result
    println!("{}", result);
}