use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn max_weight(&self, pizzas: &mut Vec<i32>) -> i64 {
        // Sort pizzas in descending order
        pizzas.sort_by(|a, b| b.cmp(a));
        
        let days = pizzas.len() / 4;
        let odd = (days + 1) / 2;
        let mut ans: i64 = 0;

        // Sum first 'odd' elements
        for i in 0..odd {
            ans += pizzas[i] as i64;
        }

        let days_half = days / 2;
        // Sum elements at positions odd + 2*i + 1 for days_half times
        for i in 0..days_half {
            let index = odd + i * 2 + 1;
            ans += pizzas[index] as i64;
        }

        ans
    }
}

fn main() {
    // Read entire input and split into whitespace tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse first token as pizza count
    let n: usize = tokens.next()
        .expect("Missing pizza count")
        .parse()
        .expect("Invalid pizza count format");

    // Parse next n tokens as pizza values
    let mut pizzas: Vec<i32> = tokens.take(n)
        .map(|s| s.parse().expect("Invalid pizza value format"))
        .collect();

    // Verify we collected exactly n values
    if pizzas.len() != n {
        panic!("Insufficient pizza values provided");
    }

    let solution = Solution;
    println!("{}", solution.max_weight(&mut pizzas));
}