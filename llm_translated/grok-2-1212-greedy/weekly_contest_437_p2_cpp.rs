use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_weight(pizzas: &mut Vec<i32>) -> i64 {
        // Sort pizzas in descending order
        pizzas.sort_by(|a, b| b.cmp(a));
        
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;
        
        // Sum the weights of pizzas for odd days
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }
        
        // Sum the weights of pizzas for even days
        for i in 0..days / 2 {
            ans += pizzas[odd + i * 2 + 1] as i64;
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of pizzas
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read pizza weights
    let mut pizzas: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the maximum weight
    let solution = Solution;
    println!("{}", solution.max_weight(&mut pizzas));

    Ok(())
}