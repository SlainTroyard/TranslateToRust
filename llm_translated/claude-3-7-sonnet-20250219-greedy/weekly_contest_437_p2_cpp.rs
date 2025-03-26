use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
        // Sort pizzas in descending order
        pizzas.sort_by(|a, b| b.cmp(a));
        
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;
        
        // Add the first 'odd' number of pizzas
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }
        
        // Add every other pizza starting from odd + 1
        for i in 0..(days / 2) {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of pizzas
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse the number of pizzas");
    
    // Read the pizza weights
    let line = lines.next().unwrap()?;
    let mut pizzas: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse().expect("Failed to parse pizza weight"))
        .collect();
    
    // Ensure we have the correct number of pizzas
    assert_eq!(pizzas.len(), n, "Number of pizzas doesn't match the specified count");
    
    // Calculate and print the result
    let solution = Solution;
    println!("{}", Solution::max_weight(&mut pizzas));
    
    Ok(())
}