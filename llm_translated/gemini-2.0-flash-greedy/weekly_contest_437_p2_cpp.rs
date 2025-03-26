use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn max_weight(&self, pizzas: &mut Vec<i32>) -> i64 {
        pizzas.sort_by(|a, b| b.cmp(a)); // Sort in descending order
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let pizzas: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let mut pizzas_mut = pizzas; // Create a mutable copy
    let solution = Solution {};
    println!("{}", solution.max_weight(&mut pizzas_mut));
}