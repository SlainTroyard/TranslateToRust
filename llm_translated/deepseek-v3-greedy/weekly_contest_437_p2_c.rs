use std::cmp::Ordering;
use std::io::{self, Write};

// Comparator function for sorting in descending order
fn cmp(a: &i32, b: &i32) -> Ordering {
    b.cmp(a)
}

fn max_weight(pizzas: &mut [i32]) -> i64 {
    let day = pizzas.len() / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort the pizzas in descending order
    pizzas.sort_by(cmp);

    let mut total = 0i64;
    let mut index = 0;

    // Sum the top `odd` pizzas
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }

    // Skip one pizza and then sum every other pizza for `even` times
    index += 1;
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }

    total
}

fn main() {
    // Read the number of pizzas
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the pizza weights
    let mut pizzas = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let weight: i32 = input.trim().parse().expect("Invalid input");
        pizzas.push(weight);
    }

    // Calculate and print the maximum weight
    let result = max_weight(&mut pizzas);
    println!("{}", result);
}