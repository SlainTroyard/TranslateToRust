use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
        // Sort in descending order (largest to smallest)
        pizzas.sort_by(|a, b| b.cmp(a));
        
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;
        
        // Add the weights of the first 'odd' pizzas
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }
        
        // Add weights of select pizzas after the first 'odd' pizzas
        for i in 0..(days / 2) {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }
        
        ans
    }
}

fn main() {
    // Set up input reading
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of pizzas
    let n: usize = lines.next()
        .expect("Failed to read input line")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse number of pizzas");
    
    // Read the pizza weights
    let mut pizzas = Vec::with_capacity(n);
    let pizzas_line = lines.next()
        .expect("Failed to read input line")
        .expect("Failed to read input");
    
    for num_str in pizzas_line.split_whitespace().take(n) {
        let pizza_weight: i32 = num_str.parse()
            .expect("Failed to parse pizza weight");
        pizzas.push(pizza_weight);
    }
    
    // Calculate and output the maximum weight
    let solution = Solution;
    println!("{}", solution.max_weight(&mut pizzas));
}