// Problem: Weekly Contest 437 Problem 2
use std::io::{self, BufRead};

fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
    // Sort the pizzas in descending order
    pizzas.sort_unstable_by(|a, b| b.cmp(a));
    
    let days = pizzas.len() / 4;
    let odd = (days + 1) / 2;
    let mut ans: i64 = 0;
    
    // Sum the first `odd` elements
    for i in 0..odd {
        ans += pizzas[i] as i64;
    }
    
    // Sum the required elements from the sorted list
    for i in 0..(days / 2) {
        ans += pizzas[odd + i * 2 + 1] as i64;
    }
    
    ans
}

fn main() {
    // Read the number of pizzas from stdin
    let n: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please type a number!")
    };

    // Read the weights of the pizzas from stdin
    let mut pizzas: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        pizzas.push(input.trim().parse().expect("Please type a number!"));
    }

    // Calculate and print the maximum weight
    let result = max_weight(&mut pizzas);
    println!("{}", result);
}