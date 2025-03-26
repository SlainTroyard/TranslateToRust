// Problem: Weekly Contest 437 Problem 2
use std::io;

fn main() {
    // Read all input at once
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Split input into tokens and parse into integers
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Extract the number of pizzas and the pizza weights
    let n = tokens[0] as usize;
    let mut pizzas = tokens[1..n + 1].to_vec();

    // Sort pizzas in descending order
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

    // Calculate the number of days and the number of odd days
    let days = pizzas.len() / 4;
    let odd = (days + 1) / 2;

    // Calculate the maximum weight
    let mut ans: i64 = 0;
    for i in 0..odd {
        ans += pizzas[i] as i64;
    }
    for i in 0..(days / 2) {
        let index = odd + i * 2 + 1;
        ans += pizzas[index] as i64;
    }

    // Output the result
    println!("{}", ans);
}