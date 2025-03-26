use std::io::{self, BufRead};
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
        // Sort pizzas in descending order
        pizzas.sort_by_key(|&x| Reverse(x));
        
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;

        // Add the largest `odd` pizzas
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }

        // Add every second pizza from the remaining days / 2
        for i in 0..(days / 2) {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of pizzas
    let n: usize = lines
        .next()
        .expect("Expected input for number of pizzas")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse number of pizzas");

    // Read the pizza weights
    let mut pizzas: Vec<i32> = lines
        .next()
        .expect("Expected input for pizza weights")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse pizza weight"))
        .collect();

    // Ensure the number of pizzas matches the input size
    assert_eq!(pizzas.len(), n, "Number of pizzas does not match input size");

    // Solve the problem
    let solution = Solution;
    let result = solution.max_weight(&mut pizzas);

    // Print the result
    println!("{}", result);
}