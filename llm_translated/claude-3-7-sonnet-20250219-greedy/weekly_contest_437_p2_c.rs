use std::io::{self, BufRead};

/// Calculates the maximum weight of pizzas that can be eaten.
///
/// # Arguments
///
/// * `pizzas` - A slice of integers representing the weights of pizzas
///
/// # Returns
///
/// The maximum total weight that can be eaten according to the rules
fn max_weight(pizzas: &mut [i32]) -> i64 {
    let pizzas_size = pizzas.len();
    let day = pizzas_size / 4;
    let even = day / 2;
    let odd = (day + 1) / 2;

    // Sort pizzas in descending order
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

    let mut total: i64 = 0;
    let mut index = 0;

    // Add the first 'odd' number of pizzas
    for _ in 0..odd {
        total += pizzas[index] as i64;
        index += 1;
    }
    
    // Skip one pizza
    index += 1;
    
    // Add every other pizza for 'even' times
    for _ in 0..even {
        total += pizzas[index] as i64;
        index += 2;
    }
    
    total
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of pizzas
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the pizza weights
    let line = lines.next().unwrap().unwrap();
    let mut pizzas: Vec<i32> = line.split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    println!("{}", max_weight(&mut pizzas));
}