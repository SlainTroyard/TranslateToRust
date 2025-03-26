// Problem: Weekly Contest 419 Problem 3

use std::io;

/// A struct to encapsulate our solution logic.
struct Solution;

impl Solution {
    /// Counts the number of winning sequences based on the input string `s`.
    /// Preserves the logic from the original C++ code exactly.
    fn count_winning_sequences(&self, s: &str) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // Map characters to numeric values: F->0, W->1, E->2
        let mut mp = [0i64; 128];
        mp[b'F' as usize] = 0;
        mp[b'W' as usize] = 1;
        mp[b'E' as usize] = 2;

        let n = s.len();
        // A 3D structure in C++ represented as vector<vector<array<int,3>>>
        // Here we store f in a 2D Vec of fixed-size arrays, each array has length 3
        // Dimensions: (n+1) x (2n+1)
        let mut f = vec![vec![[0i64; 3]; n * 2 + 1]; n + 1];

        // Initialize f[0][j] = {1,1,1} for j in [n+1..2n]
        for j in (n + 1)..=n * 2 {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1i64;

        // Main DP logic
        for i in 0..n {
            // x is the mapped value of the current character in s
            let x = mp[s.as_bytes()[i] as usize];
            // Update pow2 = 2^i, modulo MOD
            pow2 = (pow2 * 2) % MOD;

            // The j loop allows an effective index shift of n to avoid negative array indices
            // in the original code: for (int j = -i; j < n - i; j++)
            for j in -(i as i64)..(n as i64 - i as i64) {
                for ban in 0..3 {
                    // If j > i+1, directly set f[i+1][j+n][ban] = pow2
                    if j > i as i64 + 1 {
                        f[i + 1][(j + n as i64) as usize][ban] = pow2;
                        continue;
                    }

                    // We need to add from previous states
                    let res = &mut f[i + 1][(j + n as i64) as usize][ban];
                    for k in 0..3 {
                        // If this is the last character OR k != ban, we consider transitions
                        if i == n - 1 || k != ban {
                            // Compute the score = (k - x + 3) % 3
                            // If score == 2, it effectively means -1 in the original code context
                            let mut score = (k as i64 - x + 3) % 3;
                            if score == 2 {
                                score = -1;
                            }
                            let idx = j + score + n as i64;
                            *res = (*res + f[i][idx as usize][k]) % MOD;
                        }
                    }
                }
            }
        }

        // The final answer is f[n][n][0] modulo 1_000_000_007
        // which is guaranteed to be within i64 range, so casting to i32 is safe
        f[n][n][0] as i32
    }
}

fn main() {
    // Read the integer "count" from stdin (not used in logic, but we must preserve input format)
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line for count");
    let _count: i32 = input_str.trim().parse().expect("Failed to parse integer");

    // Read the string s from stdin
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line for string");
    let s = s.trim().to_string();

    // Instantiate the solution and compute the result
    let solution = Solution;
    let result = solution.count_winning_sequences(&s);

    // Print the result to stdout
    println!("{}", result);
}