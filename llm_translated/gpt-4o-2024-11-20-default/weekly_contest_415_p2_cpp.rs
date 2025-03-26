use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_score(a: Vec<i64>, b: Vec<i64>) -> i64 {
        // Array to store scores for 0 to 4 steps
        let mut f = [0, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2];

        for &y in &b {
            // Traverse backwards to ensure we're not overwriting the values we are calculating
            for j in (0..4).rev() {
                f[j + 1] = max(f[j + 1], f[j] + a[j] * y);
            }
        }

        // Return the maximum score after the complete computation for 4 steps
        f[4]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read sizes of arrays `a` and `b`
    let sizes_line = lines.next().unwrap().unwrap();
    let sizes: Vec<usize> = sizes_line
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let a_size = sizes[0];
    let b_size = sizes[1];

    // Read array `a`
    let a_line = lines.next().unwrap().unwrap();
    let a: Vec<i64> = a_line
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    assert_eq!(a.len(), a_size);

    // Read array `b`
    let b_line = lines.next().unwrap().unwrap();
    let b: Vec<i64> = b_line
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    assert_eq!(b.len(), b_size);

    // Create instance of `Solution` and compute the result
    let solution = Solution;
    let result = solution.max_score(a, b);

    // Output the result
    println!("{}", result);
}