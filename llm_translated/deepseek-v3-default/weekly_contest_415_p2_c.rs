use std::cmp::max;
use std::io::{self, BufRead};

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];
    
    f[0][0] = 0;
    
    for i in 1..=n {
        for j in 0..=4 {
            f[i][j] = f[i - 1][j];
            if j > 0 {
                f[i][j] = max(f[i][j], f[i - 1][j - 1] + a[j - 1] as i64 * b[i - 1] as i64);
            }
        }
    }
    
    f[n][4]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line for aSize and bSize
    let first_line = lines.next().unwrap().unwrap();
    let sizes: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let a_size = sizes[0];
    let b_size = sizes[1];
    
    // Read the second line for array a
    let second_line = lines.next().unwrap().unwrap();
    let a: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read the third line for array b
    let third_line = lines.next().unwrap().unwrap();
    let b: Vec<i32> = third_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    println!("{}", max_score(&a, &b));
}