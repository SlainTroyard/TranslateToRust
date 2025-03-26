use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {
    // Read the number of pizzas
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    // Read the weights of the pizzas
    let mut pizzas = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let weight: i32 = input.trim().parse().expect("Please enter a valid number");
        pizzas.push(weight);
    }

    // Calculate the maximum weight
    let solution = Solution;
    let max_weight = solution.max_weight(&mut pizzas);

    // Print the result
    println!("{}", max_weight);
}

struct Solution;

impl Solution {
    pub fn max_weight(&self, pizzas: &mut Vec<i32>) -> i64 {
        // Sort the pizzas in descending order
        pizzas.sort_by(|a, b| b.cmp(a));

        // Calculate the number of days and the number of odd days
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;

        // Initialize the answer
        let mut ans: i64 = 0;

        // Sum the weights of the pizzas for the odd days
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }

        // Sum the weights of the pizzas for the even days
        for i in 0..days / 2 {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }

        ans
    }
}