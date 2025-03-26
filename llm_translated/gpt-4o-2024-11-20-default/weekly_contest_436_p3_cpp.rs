// Problem: Weekly Contest 436 Problem 3
/// This program translates the provided C++ solution into idiomatic Rust.
/// The implementation directly mirrors the algorithm's logic while adhering
/// to Rust's safety and type system.

use std::io;

struct Solution;

impl Solution {
    fn count_substrings(s: &str) -> i64 {
        let mut ans: i64 = 0;
        let mut f = [[0; 9]; 10]; // Equivalent to `array<array<int, 9>, 10>` in C++
        
        for c in s.chars() {
            let d = c.to_digit(10).expect("Invalid character!") as usize; // Convert char to digit
            for m in 1..10 {
                let mut nf = [0; 9]; // Temporary array to store the new values
                nf[d % m] = 1; // Initialize based on the current digit modulo `m`
                
                for rem in 0..m {
                    nf[(rem * 10 + d) % m] += f[m][rem]; // Update `nf` values based on prior state
                }
                
                f[m] = nf; // Commit the updates to the main array
            }
            ans += f[d][0]; // Accumulate the results for the current digit
        }
        
        ans
    }
}

fn main() {
    // Input handling
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let s = input.trim(); // Read and trim the string input
    
    let sol = Solution;
    let result = sol.count_substrings(s);
    
    // Output the result
    println!("{}", result);
}