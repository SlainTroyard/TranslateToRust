use std::io::{self, Read};
use std::process;

fn max_distance(s: &str, k: i32) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut ans = 0;

    for (i, c) in s.chars().enumerate() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => {} // Assume input only contains valid directions per problem constraints
        }

        // Calculate current maximum possible distance considering:
        // 1. Current Manhattan distance + maximum possible gain from k steps (2 per step)
        // 2. Total steps taken so far (i+1) as the upper bound for distance
        let current_max = (x.abs() + y.abs() + 2 * k).min(i as i32 + 1);
        ans = ans.max(current_max);
    }

    ans
}

fn main() {
    // Read entire input to handle cases where values are split across lines
    let input = {
        let mut input = String::new();
        if let Err(_) = io::stdin().read_to_string(&mut input) {
            eprintln!("Error reading input");
            process::exit(1);
        }
        input
    };

    // Split input into whitespace-separated tokens (matching original scanf behavior)
    let mut parts = input.split_whitespace();
    
    // Parse string part (first token)
    let s = parts.next().unwrap_or_else(|| {
        eprintln!("Error reading input");
        process::exit(1);
    });

    // Parse integer k (second token)
    let k_str = parts.next().unwrap_or_else(|| {
        eprintln!("Error reading input");
        process::exit(1);
    });

    let k: i32 = match k_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error reading input");
            process::exit(1);
        }
    };

    // Calculate and print result
    let result = max_distance(s, k);
    println!("{}", result);
}