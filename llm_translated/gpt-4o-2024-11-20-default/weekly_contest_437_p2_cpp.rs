// Problem: Weekly Contest 437 Problem 2
use std::cmp::Reverse;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_weight(pizzas: Vec<i32>) -> i64 {
        // Sort pizzas in descending order
        let mut pizzas = pizzas;
        pizzas.sort_by_key(|&x| Reverse(x));

        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;

        // Sum weights of top `odd` pizzas
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }

        // Sum weights in the pattern defined by the formula
        for i in 0..(days / 2) {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }

        ans
    }
}

fn main() {
    // Read input: n followed by n integers
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line to get the number of pizzas
    let n: usize = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected a number");

    // Parse the second line to get the pizzas weights
    let pizzas: Vec<i32> = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected numbers"))
        .collect();

    // Ensure the number of weights matches n
    assert_eq!(pizzas.len(), n, "Number of weights does not match n");

    // Compute the result using the solution
    let solution = Solution;
    let result = solution.max_weight(pizzas);

    // Output the result
    println!("{}", result);
}