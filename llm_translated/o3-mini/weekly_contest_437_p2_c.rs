use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin into a string.
    // The original C code uses scanf so it can handle multiple whitespace-separated numbers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input based on whitespace.
    let mut tokens = input.split_whitespace();

    // Read the first integer "n", the number of pizzas.
    let n_str = tokens.next().ok_or("Expected the number of pizzas as first input")?;
    let n: usize = n_str.parse()?;

    // Allocate a vector to store the pizza weights.
    let mut pizzas = Vec::with_capacity(n);
    for _ in 0..n {
        let token = tokens.next().ok_or("Expected a pizza weight")?;
        let weight: i32 = token.parse()?;
        pizzas.push(weight);
    }

    // Calculate the maximum weight following the algorithm.
    let result = max_weight(&mut pizzas);
    
    // Print the result exactly as in the C code.
    println!("{}", result);
    Ok(())
}

/// Computes the maximum weight as defined in LeetCode Weekly Contest 437 Problem 2.
///
/// The algorithm works as follows:
/// 1. Compute day = pizzas_size / 4.
/// 2. Compute odd = (day + 1)/2 and even = day/2.
/// 3. Sort the pizzas in descending order.
/// 4. Sum the first odd number of pizzas.
/// 5. Skip one pizza, then add every second pizza for the even count.
fn max_weight(pizzas: &mut [i32]) -> i64 {
    let pizzas_size = pizzas.len();
    let day = pizzas_size / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort in descending order.
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

    let mut total: i64 = 0;
    let mut index = 0;

    // Sum the first odd pizzas.
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }

    // Skip one pizza (as in index++ in the C code).
    index += 1;

    // Then add every second pizza for the even count.
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }

    total
}