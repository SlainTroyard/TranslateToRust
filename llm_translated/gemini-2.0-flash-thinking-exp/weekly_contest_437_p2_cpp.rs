// Problem: Weekly Contest 437 Problem 2
use std::io;
use std::vec::Vec;
use std::cmp::Reverse;

// Function to calculate the maximum weight of pizzas
fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
    // Sort the pizzas in descending order
    pizzas.sort_by_key(|&x| Reverse(x));

    // Calculate the number of days based on the problem logic
    let days = pizzas.len() / 4;
    // Calculate 'odd' value as per the original C++ code
    let odd = (days + 1) / 2;

    let mut ans: i64 = 0;

    // Sum the first 'odd' largest pizza weights
    for i in 0..odd {
        ans += pizzas[i] as i64;
    }

    // Sum pizza weights at specific indices based on 'odd' and 'days'
    for i in 0..days / 2 {
        ans += pizzas[odd + i * 2 + 1] as i64;
    }

    ans
}

fn main() -> io::Result<()> {
    // Read the number of pizzas (n) from stdin
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str)?;
    let n: usize = n_str.trim().parse().unwrap();

    // Create a vector to store pizza weights
    let mut pizzas: Vec<i32> = Vec::with_capacity(n);

    // Read pizza weights from stdin and populate the vector
    let mut pizza_weights_str = String::new();
    io::stdin().read_line(&mut pizza_weights_str)?;
    let weights_iter = pizza_weights_str.trim().split_whitespace();
    for weight_str in weights_iter {
        let weight: i32 = weight_str.parse().unwrap();
        pizzas.push(weight);
    }

    // Call the max_weight function to calculate the result
    let result = max_weight(&mut pizzas);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}