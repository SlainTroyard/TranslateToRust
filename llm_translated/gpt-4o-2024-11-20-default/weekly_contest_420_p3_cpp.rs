// Problem: Weekly Contest 420 Problem 3 translated from C++ to Rust

use std::io::{self, BufRead};

const MX: usize = 1_000_001;

// LPF[i] stores the Lowest Prime Factor of the number i
static mut LPF: [usize; MX] = [0; MX];

// Initialize the LPF array.
// This is equivalent to the C++ static initialization using a lambda.
fn initialize_lpf() {
    unsafe {
        for i in 2..MX {
            if LPF[i] == 0 {
                for j in (i..MX).step_by(i) {
                    if LPF[j] == 0 {
                        LPF[j] = i;
                    }
                }
            }
        }
    }
}

struct Solution;

impl Solution {
    // Translated version of `minOperations` from C++ to Rust.
    pub fn min_operations(nums: &mut Vec<usize>) -> i32 {
        let mut ans = 0;
        unsafe {
            for i in (0..nums.len() - 1).rev() {
                if nums[i] > nums[i + 1] {
                    nums[i] = LPF[nums[i]];
                    if nums[i] > nums[i + 1] {
                        return -1;
                    }
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    // Initialize LPF array (equivalent to static lambda initialization in C++).
    initialize_lpf();

    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines
        .next()
        .expect("Expected a line for 'n'")
        .expect("Failed to read line for 'n'")
        .trim()
        .parse()
        .expect("'n' should be a valid integer");
    
    let nums: Vec<usize> = lines
        .next()
        .expect("Expected a line for 'nums'")
        .expect("Failed to read line for 'nums'")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("'nums' should contain valid integers"))
        .collect();

    assert_eq!(nums.len(), n, "'nums' length should be equal to n");

    let mut nums = nums; // Mutable version of `nums` for passing to `min_operations`.

    // Solve the problem
    let solution = Solution;
    let result = solution.min_operations(&mut nums);

    // Output the result
    println!("{}", result);
}