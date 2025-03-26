use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn max_weight(&self, pizzas: &mut Vec<i32>) -> i64 {
        pizzas.sort_by(|a, b| b.cmp(a));
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }
        for i in 0..(days / 2) {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }
        return ans;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let pizzas: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut pizzas_mut = pizzas; // Create a mutable copy for sorting
    let solution = Solution {};
    println!("{}", solution.max_weight(&mut pizzas_mut));
}