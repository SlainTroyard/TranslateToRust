use std::cmp::max;
use std::io;
use std::i64;

struct Solution {}

impl Solution {
    pub fn max_score(a: Vec<int>, b: Vec<int>) -> i64 {
        let mut f: [i64; 5] = [0; 5];
        for i in 1..5 {
            f[i] = i64::MIN / 2;
        }
        for &y in &b {
            for j in (0..4).rev() {
                f[j + 1] = max(f[j + 1], f[j] + a[j] as i64 * y as i64);
            }
        }
        f[4]
    }
}

fn main() {
    let mut a_size = String::new();
    io::stdin().read_line(&mut a_size).unwrap();
    let a_size: usize = a_size.trim().parse().unwrap();

    let mut b_size = String::new();
    io::stdin().read_line(&mut b_size).unwrap();
    let b_size: usize = b_size.trim().parse().unwrap();

    let mut a_line = String::new();
    io::stdin().read_line(&mut a_line).unwrap();
    let a: Vec<int> = a_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut b_line = String::new();
    io::stdin().read_line(&mut b_line).unwrap();
    let b: Vec<int> = b_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution {};
    println!("{}", solution.max_score(a, b));
}