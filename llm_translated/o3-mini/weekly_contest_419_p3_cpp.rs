use std::error::Error;
use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    // This function translates the C++ countWinningSequences method to idiomatic Rust.
    fn count_winning_sequences(s: &str) -> i64 {
        let n = s.len();
        // Convert the input string into bytes since we are dealing with ASCII characters.
        let s_bytes = s.as_bytes();
        // Mapping for characters: 'F' => 0, 'W' => 1, 'E' => 2.
        let mut mp = [0; 128];
        mp[b'F' as usize] = 0;
        mp[b'W' as usize] = 1;
        mp[b'E' as usize] = 2;

        // Create a DP table f with dimensions (n+1) x (2*n+1), each cell is an array of 3 i64.
        let mut f = vec![vec!([0; 3]; n * 2 + 1); n + 1];

        // Initialization based on the C++ code.
        // For j from n+1 to 2*n, set f[0][j] = [1, 1, 1]
        for j in (n + 1)..=(n * 2) {
            f[0][j] = [1, 1, 1];
        }

        let mut pow2 = 1;
        // Process each character in the string.
        for i in 0..n {
            // Get the mapped value for the character at position i.
            let x = mp[s_bytes[i] as usize];
            // Update pow2 = 2^(i+1) % MOD
            pow2 = (pow2 * 2) % MOD;
            // We use j to represent the balance offset; j ranges from -i to (n-i-1)
            // Use i32 here for negative values.
            for j in -(i as i32)..(n as i32 - i as i32) {
                // Compute the index adjustment offset.
                let j_idx = (j + n as i32) as usize;
                // For each "ban" value from 0 to 2.
                for ban in 0..3 {
                    // If j is greater than i+1, we can fill the DP state directly.
                    if j > i as i32 + 1 {
                        f[i + 1][j_idx][ban] = pow2;
                        continue;
                    }
                    // Otherwise, we need to accumulate the transitions.
                    // Using a mutable reference for clarity.
                    let res = &mut f[i + 1][j_idx][ban];
                    // Iterate over possible k values (0, 1, 2).
                    for k in 0..3 {
                        // When we are at the last character or k is not banned.
                        if i == n - 1 || k != ban {
                            // Compute score = (k - x + 3) % 3.
                            // If score equals 2, then it represents -1.
                            let mut score = (k as i32 - x as i32 + 3) % 3;
                            if score == 2 {
                                score = -1;
                            }
                            // Calculate the corresponding index in the DP table.
                            let prev_j = j + score;
                            let prev_j_idx = (prev_j + n as i32) as usize;
                            *res = (*res + f[i][prev_j_idx][k]) % MOD;
                        }
                    }
                }
            }
        }
        // Return the result from the DP table.
        f[n][n][0]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input into tokens (handles multiple lines or values per line).
    let mut tokens = input.split_whitespace();

    // Read the first token as an integer (variable "count" from C++ code).
    let _count: i32 = tokens
        .next()
        .ok_or("Expected an integer for count")?
        .parse()?;
    // Read the next token as the string "s".
    let s = tokens
        .next()
        .ok_or("Expected a string for s")?
        .to_string();

    // Compute the result using our Solution.
    let result = Solution::count_winning_sequences(&s);
    // Print the result.
    println!("{}", result);
    Ok(())
}