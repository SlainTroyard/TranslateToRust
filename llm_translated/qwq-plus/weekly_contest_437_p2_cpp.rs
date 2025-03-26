use std::io;

/// Translated solution for LeetCode Weekly Contest 437 Problem 2.
///
/// Reads pizza weights from stdin, computes the maximum possible weight
/// following the specified algorithm, and outputs the result.
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut tokens = input.split_whitespace();
    let n: usize = tokens
        .next()
        .expect("Missing n")
        .parse()
        .expect("Invalid n value");

    let pizzas: Vec<i32> = tokens
        .map(|s| s.parse().expect("Invalid pizza weight"))
        .take(n)
        .collect();

    let result = max_weight(pizzas);
    println!("{}", result);
}

/// Computes the maximum weight according to the problem's algorithm.
///
/// # Arguments
///
/// * `pizzas` - A vector of pizza weights.
///
/// # Returns
///
/// * The computed maximum weight as an i64.
fn max_weight(mut pizzas: Vec<i32>) -> i64 {
    // Sort pizzas in descending order
    pizzas.sort_unstable_by(|a, b| b.cmp(a));

    let days = pizzas.len() / 4;
    let odd = (days + 1) / 2;
    let mut ans: i64 = 0;

    // Add the first 'odd' elements
    for &p in &pizzas[0..odd] {
        ans += p as i64;
    }

    // Add elements at positions odd + i*2 +1 for i in 0..(days/2)
    for i in 0..(days / 2) {
        let idx = odd + i * 2 + 1;
        ans += pizzas[idx] as i64;
    }

    ans
}