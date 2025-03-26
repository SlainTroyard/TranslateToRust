use std::io::{self, BufRead};

/// A struct to mirror the C++ `Solution` class.
struct Solution;

impl Solution {
    /// Replicates the logic of the original C++ maxScore function.
    fn max_score(&self, a: &[i32], b: &[i32]) -> i64 {
        // f will hold intermediate states, just like in the C++ code.
        // f[0] = 0, and fill the rest with i64::MIN/2 as per LLONG_MIN/2 in C++.
        let mut f = [
            0_i64,
            i64::MIN / 2,
            i64::MIN / 2,
            i64::MIN / 2,
            i64::MIN / 2,
        ];
        
        // For each element y in b, update f in reverse order to avoid overwriting.
        for &y in b {
            for j in (0..=3).rev() {
                let candidate = f[j] + (a[j] as i64) * (y as i64);
                if candidate > f[j + 1] {
                    f[j + 1] = candidate;
                }
            }
        }
        // The answer is the 5th element in f (index 4).
        f[4]
    }
}

fn main() -> io::Result<()> {
    // We'll read all tokens from stdin, then parse exactly in the order of:
    //  aSize, bSize, followed by aSize items for a, and bSize items for b.
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Read all lines until exhaustion, collecting integer tokens.
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        for word in line.split_whitespace() {
            if let Ok(num) = word.parse::<i32>() {
                tokens.push(num);
            }
        }
    }

    // We expect at least two tokens for aSize and bSize.
    // Then aSize for a, bSize for b.
    // The original code does not handle invalid input, so we'll trust the input here.
    let mut index = 0;
    let a_size = tokens[index] as usize; 
    index += 1;
    let b_size = tokens[index] as usize; 
    index += 1;

    // Parse a.
    let mut a = Vec::with_capacity(a_size);
    for _ in 0..a_size {
        a.push(tokens[index]);
        index += 1;
    }

    // Parse b.
    let mut b = Vec::with_capacity(b_size);
    for _ in 0..b_size {
        b.push(tokens[index]);
        index += 1;
    }

    // Create a Solution instance and compute the result.
    let solution = Solution;
    let result = solution.max_score(&a, &b);

    // Output the result to stdout (no extra formatting).
    println!("{}", result);

    Ok(())
}