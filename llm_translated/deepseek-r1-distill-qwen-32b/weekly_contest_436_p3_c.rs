// Problem: Weekly Contest 436 Problem 3
use std::io;

/// Counts the number of substrings in `s` that represent numbers divisible by their last digit.
fn count_substrings(s: &str) -> i64 {
    let mut ans = 0;
    let len = s.len();
    // f[m][rem] represents the count of substrings modulo m with remainder `rem`
    let mut f = vec![vec![0; 9]; 10]; // m ranges from 1 to 9, rem from 0 to m-1
    
    for c in s.chars() {
        let d = c.to_digit(10).unwrap() as usize; // Current digit
        
        for m in 1..10 {
            // Temporary array to store new counts for modulus m
            let mut nf = vec![0; 9];
            // Single digit case
            nf[d % m] += 1;
            
            // Update counts based on previous remainders
            for rem in 0..m {
                let new_rem = (rem * 10 + d) % m;
                nf[new_rem] += f[m][rem];
            }
            
            // Update f[m] with new counts
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }
        
        // Add substrings ending at current position that are divisible by d
        ans += f[d][0];
    }
    
    ans as i64
}

fn main() {
    // Read input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n == 0 {
                println!("0");
                return;
            }
            // Remove newline character
            let s = input.trim_end();
            if s.is_empty() {
                println!("0");
                return;
            }
            
            // Compute result
            let result = count_substrings(s);
            
            // Output result
            println!("{}", result);
        },
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            std::process::exit(1);
        }
    }
}