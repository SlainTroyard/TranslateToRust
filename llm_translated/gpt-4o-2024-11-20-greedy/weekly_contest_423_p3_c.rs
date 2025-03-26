use std::io::{self, BufRead};
use std::cmp::max;

const MOD: i64 = 1_000_000_007;

fn sum_of_good_subsequences(nums: &[i32]) -> i64 {
    let mut cnt = vec![0i64; 100_003];
    let mut sum = vec![0i64; 100_003];
    let mut ans = 0i64;

    for &num in nums {
        let x = num + 1; // Increment the number as in the original C code
        let c = cnt[(x - 1) as usize] + 1 + cnt[(x + 1) as usize];
        cnt[x as usize] = (cnt[x as usize] + c) % MOD;

        let s = c * (x - 1) as i64 + sum[(x - 1) as usize] + sum[(x + 1) as usize];
        sum[x as usize] = (sum[x as usize] + s) % MOD;

        ans = (ans + s) % MOD;
    }

    ans
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .expect("Expected input for array size")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse array size");

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for array elements")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    assert_eq!(nums.len(), n, "Array size does not match the number of elements provided");

    // Call the function and output the result
    let result = sum_of_good_subsequences(&nums);
    println!("{}", result);
}