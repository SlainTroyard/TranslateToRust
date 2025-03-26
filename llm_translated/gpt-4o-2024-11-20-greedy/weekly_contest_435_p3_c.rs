use std::io::{self, BufRead};
use std::cmp::min;
use std::collections::VecDeque;

// Function to calculate the greatest common divisor (GCD)
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Main function to solve the problem
fn minimum_increments(nums: &[i64], target: &[i64]) -> i64 {
    let n = nums.len();
    let m = target.len();
    let inf: i64 = 1e18 as i64;

    // Compute the `g` array for divisibility calculations
    let mut g = vec![1i64; 1 << m];
    for i in 0..(1 << m) {
        for j in 0..m {
            if (i >> j) & 1 == 1 {
                g[i] = g[i] / gcd(g[i], target[j]) * target[j];
            }
        }
    }

    // Dynamic programming array
    let mut f = vec![vec![inf; 1 << m]; 2];
    f[0][0] = 0;

    // Dynamic programming process
    for i in 1..=n {
        let curr = i & 1;
        let prev = curr ^ 1;

        // Copy previous state
        f[curr].copy_from_slice(&f[prev]);

        for j in 0..(1 << m) {
            let mut k = j;
            while k > 0 {
                let v = ((nums[i - 1] + g[k] - 1) / g[k]) * g[k] - nums[i - 1];
                f[curr][j] = min(f[curr][j], f[prev][j ^ k] + v);
                k = (k - 1) & j;
            }
        }
    }

    f[n & 1][(1 << m) - 1]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let m: usize = first_iter.next().unwrap().parse().unwrap();

    // Read nums array
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i64> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read target array
    let third_line = lines.next().unwrap().unwrap();
    let target: Vec<i64> = third_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure input sizes match
    assert_eq!(nums.len(), n);
    assert_eq!(target.len(), m);

    // Call the function and print the result
    let result = minimum_increments(&nums, &target);
    println!("{}", result);
}