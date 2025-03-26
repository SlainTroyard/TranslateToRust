use std::io;
use std::str;

struct Solution;

impl Solution {
    fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut f = [0i64; 5];
        for j in 1..5 {
            f[j] = i64::MIN / 2;
        }
        for y in b {
            for j in (0..=3).rev() {
                f[j + 1] = f[j + 1].max(f[j] + (a[j] as i64) * (y as i64));
            }
        }
        f[4]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(str::parse::<i32>)
        .collect::<Result<_, _>>()
        .unwrap();
    let a_size = tokens[0] as usize;
    let b_size = tokens[1] as usize;
    let a = tokens[2..2 + a_size].to_vec();
    let b = tokens[2 + a_size..2 + a_size + b_size].to_vec();
    let solution = Solution;
    println!("{}", solution.max_score(a, b));
}