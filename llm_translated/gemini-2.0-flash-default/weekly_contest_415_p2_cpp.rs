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
    let mut a_size_str = String::new();
    io::stdin()
        .read_line(&mut a_size_str)
        .expect("Failed to read line");
    let mut a_size = a_size_str.trim().split_whitespace();

    let a_size: usize = a_size.next().unwrap().parse().unwrap();

    let mut b_size_str = String::new();
    io::stdin()
        .read_line(&mut b_size_str)
        .expect("Failed to read line");

    let mut b_size = b_size_str.trim().split_whitespace();

    let b_size: usize = b_size.next().unwrap().parse().unwrap();

    let mut a_str = String::new();
    io::stdin()
        .read_line(&mut a_str)
        .expect("Failed to read line");

    let a: Vec<i32> = a_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut b_str = String::new();
    io::stdin()
        .read_line(&mut b_str)
        .expect("Failed to read line");

    let b: Vec<i32> = b_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution {};
    println!("{}", solution.max_score(a, b));
}