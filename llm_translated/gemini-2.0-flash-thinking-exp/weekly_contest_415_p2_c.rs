use std::cmp::max;
use std::io;

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];

    for i in 0..=n {
        for j in 0..=4 {
            f[i][j] = i64::MIN;
        }
    }
    f[0][0] = 0;

    for i in 1..=n {
        for j in 0..=4 {
            f[i][j] = f[i - 1][j]; // Option 1: Don't use b[i-1] in this step
            if j > 0 {
                // Option 2: Use b[i-1] and a[j-1]
                f[i][j] = max(f[i][j], f[i - 1][j - 1] + (a[j - 1] as i64) * (b[i - 1] as i64));
            }
        }
    }
    f[n][4]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    let a_size: usize = iter.next().unwrap().parse().expect("Failed to parse aSize");
    let b_size: usize = iter.next().unwrap().parse().expect("Failed to parse bSize");

    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).expect("Failed to read line");
    let a: Vec<i32> = a_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse a element"))
        .collect();

    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).expect("Failed to read line");
    let b: Vec<i32> = b_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse b element"))
        .collect();

    println!("{}", max_score(&a, &b));
}