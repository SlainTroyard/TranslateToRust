use std::io::{self, BufRead};
use std::cmp::Reverse;

fn main() {
    // Read the number of pizzas
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the weights of the pizzas
    let mut pizzas: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let weight: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        pizzas.push(weight);
    }

    // Calculate the maximum weight
    let max_weight = max_weight(&mut pizzas);
    println!("{}", max_weight);
}

fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
    // Sort the pizzas in descending order
    pizzas.sort_by_key(|&x| Reverse(x));

    // Calculate the number of days and the number of odd days
    let days = pizzas.len() / 4;
    let odd = (days + 1) / 2;

    // Calculate the maximum weight
    let mut ans: i64 = 0;
    for i in 0..odd {
        ans += pizzas[i] as i64;
    }
    for i in 0..days / 2 {
        ans += pizzas[odd + i * 2 + 1] as i64;
    }

    ans
}