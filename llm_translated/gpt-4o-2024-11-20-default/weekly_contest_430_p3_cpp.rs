```rust
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::num::ParseIntError;

// Helper function to compute gcd
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

struct Solution;

impl Solution {
    pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf = HashMap::new();

        // Compute the 'suf' hashmap for suffix pairs
        for i in 4..n - 2 {
            let c = nums[i];
            for j in (i + 2)..n {
                let d = nums[j];
                let g = gcd(c, d);
                *suf
                    .entry(((d / g) << 16) | (c / g))
                    .or_insert(0) += 1;
            }
        }

        let mut ans = 0_i64;

        // Main loop to compute the result using the prefix pairs
        for i in 2..n - 4 {
            let b = nums[i];
            for j in 0..i - 1 {
                let a = nums[j];
                let g = gcd(a, b);
                ans += suf
                    .get(&(((a / g) << 16) | (b / g)))
                    .unwrap_or(&0) as &i32 as &.  
