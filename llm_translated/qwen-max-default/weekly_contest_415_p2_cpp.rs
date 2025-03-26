use std::io::{self, BufRead, Write};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the sizes of vectors a and b
    let sizes: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (a_size, b_size) = (sizes[0], sizes[1]);

    // Parse vector a
    let a: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Parse vector b
    let b: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Create an instance of Solution and compute the result
    let solution = Solution;
    let result = solution.max_score(&a, &b);

    // Output the result to stdout
    writeln!(io::stdout(), "{}", result).unwrap();
}

struct Solution;

impl Solution {
    pub fn max_score(a: &[i32], b: &[i32]) -> i64 {
        // Initialize the dp array with very small values
        let mut f: [i64; 5] = [0; 5];
        for i in 1..5 {
            f[i] = i64::MIN / 2;
        }

        // Iterate over each element in b
        for &y in b.iter() {
            // Update the dp array
            for j in (0..=3).rev() {
                f[j + 1] = f[j + 1].max(f[j] + (a[j as usize] as i64 * y as i64));
            }
        }

        // Return the final result
        f[4]
    }
}