use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut ans = 0i64;
        let mut f = [[0; 9]; 10]; // 2D array to store intermediate results

        for c in s.chars() {
            let d = c as usize - '0' as usize; // Convert character to digit
            for m in 1..10 {
                let mut nf = [0; 9]; // Temporary array for the new state
                nf[d % m] = 1; // Initialize the current digit's contribution
                for rem in 0..m {
                    nf[(rem * 10 + d) % m] += f[m][rem];
                }
                f[m] = nf; // Update the state for modulus m
            }
            ans += f[d][0]; // Add the count of valid substrings for modulus d
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim().to_string();

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.count_substrings(s);

    // Print the result to stdout
    println!("{}", result);
}