use std::io::{self, BufRead};

/// Represents the solution for the problem.
struct Solution;

impl Solution {
    /// Calculates the maximum weight of pizzas based on the given algorithm.
    fn max_weight(pizzas: &Vec<i32>) -> i64 {
        let mut pizzas = pizzas.clone();
        pizzas.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;
        
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }
        for i in 0..(days / 2) {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }
        
        ans
    }
}

fn main() {
    // Read the number of pizzas from stdin
    let n: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please type a number!")
    };

    // Read the weights of the pizzas from stdin
    let mut pizzas = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let weight: i32 = input.trim().parse().expect("Please type a number!");
        pizzas.push(weight);
    }

    // Create an instance of the Solution and calculate the max weight
    let solution = Solution;
    let result = Solution::max_weight(&pizzas);

    // Output the result to stdout
    println!("{}", result);
}