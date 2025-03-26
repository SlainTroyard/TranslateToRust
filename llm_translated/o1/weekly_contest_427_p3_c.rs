// Translated from C to Rust for LeetCode Weekly Contest 427 Problem 3
// Note: This program preserves the same algorithm and I/O format as the original C code.

use std::io::{self, BufRead};

// A simple Scanner struct to replicate C's behavior of reading tokens
// in any order (including multiple on the same line or one per line).
struct Scanner {
    tokens: Vec<String>,
    index: usize,
}

impl Scanner {
    fn new() -> Self {
        // Read all stdin into a string, then split by whitespace.
        let mut input = String::new();
        io::stdin()
            .lock()
            .read_to_string(&mut input)
            .expect("Failed to read from stdin");

        let tokens: Vec<String> = input.split_whitespace().map(str::to_string).collect();
        Scanner { tokens, index: 0 }
    }

    // Reads the next token as i32. Panics on parsing error or insufficient input.
    fn next_i32(&mut self) -> i32 {
        let val = self.tokens[self.index].parse().expect("Failed to parse integer");
        self.index += 1;
        val
    }
}

// Returns the maximum of two i64 values
fn max_value(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

// Translated from the C function maxSubarraySum(int* nums, int n, int k)
fn max_subarray_sum(nums: &[i32], n: usize, k: usize) -> i64 {
    // keep will hold the sum of every subarray of length k
    let mut keep = vec![0_i64; n - k + 1];
    let mut cur = 0_i64;

    // Fill keep[] with subarray sums of length k in a sliding window manner
    for i in 0..n {
        cur += nums[i] as i64;
        if i >= k - 1 {
            keep[i - (k - 1)] = cur;
            cur -= nums[i - (k - 1)] as i64;
        }
    }

    // Now compute the maximum possible sum with step size k
    let mut ans = i64::MIN;
    for i in 0..k.min(n - k + 1) {
        let mut subseq_sum = 0_i64;
        let mut local_max = keep[i];
        let mut l = i;
        // Summation with jump size k
        while l < (n - k + 1) {
            subseq_sum += keep[l];
            if subseq_sum > local_max {
                local_max = subseq_sum;
            }
            if subseq_sum < 0 {
                subseq_sum = 0;
            }
            l += k;
        }
        ans = max_value(ans, local_max);
    }

    ans
}

fn main() {
    // Exactly replicate the original input pattern:
    // 1) Read n
    // 2) Read k
    // 3) Read n integers
    let mut sc = Scanner::new();
    let n = sc.next_i32() as usize;
    let k = sc.next_i32() as usize;

    let mut nums = vec![0_i32; n];
    for i in 0..n {
        nums[i] = sc.next_i32();
    }

    // Calculate and print the result (single line, just the value)
    let result = max_subarray_sum(&nums, n, k);
    println!("{}", result);
}