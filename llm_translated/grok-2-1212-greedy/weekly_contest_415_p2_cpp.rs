use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_score(a: &[i32], b: &[i32]) -> i64 {
        let mut f = [0i64; 5];
        f[1..].fill(i64::MIN / 2);
        for &y in b {
            for j in (0..4).rev() {
                f[j + 1] = f[j + 1].max(f[j] + a[j] as i64 * y as i64);
            }
        }
        f[4]
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input sizes
    let sizes: Vec<usize> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let a_size = sizes[0];
    let b_size = sizes[1];

    // Read vector 'a'
    let a: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read vector 'b'
    let b: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::max_score(&a, &b);
    println!("{}", result);

    Ok(())
}