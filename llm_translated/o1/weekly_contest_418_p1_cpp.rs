// LeetCode Weekly Contest 418 Problem 1 in Rust
//
// REQUIREMENTS FULFILLED:
// 1. Translates the entire C++ code, including main and I/O, into Rust.
// 2. Preserves the exact same logic and algorithm as the original code.
// 3. Uses idiomatic Rust with basic error handling.
// 4. Maintains the exact same stdin/stdout format.
// 5. The program reads a size, then reads that many integers, and outputs the result.

use std::io::{self, BufRead};
use std::cmp::Ordering;

// Helper function to compute the bit length of a positive i32.
// In C++: __lg(x) + 1 is effectively the number of bits to represent x (without leading zeros).
// For example, if x = 10, __lg(10) = 3, plus 1 = 4 bits to represent 10.
fn bit_length(x: i32) -> u32 {
    // For x > 0, 32 - x.leading_zeros() is the number of bits used to represent x.
    32 - x.leading_zeros()
}

// Implementation of the original "Solution" class.
struct Solution;

impl Solution {
    // Replicates maxGoodNumber exactly.
    fn max_good_number(&self, nums: &mut [i32]) -> i32 {
        // Sort with the custom comparator:
        // C++: sort by whether (a << len_b | b) > (b << len_a | a).
        // We want the larger combination first, so we return Ordering::Less if the combo is larger
        // (which in Rust means 'a' should come before 'b' for a descending sort).
        nums.sort_by(|&a, &b| {
            let len_a = bit_length(a);
            let len_b = bit_length(b);
            let ab = ((a as i64) << len_b) | (b as i64);
            let ba = ((b as i64) << len_a) | (a as i64);
            if ab > ba {
                Ordering::Less
            } else if ab < ba {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        // Accumulate the answer by shifting and OR-ing as in the original code.
        let mut ans: i32 = 0;
        for &x in nums.iter() {
            ans = (ans << bit_length(x)) | x;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let num_size_line = match lines.next() {
        Some(line) => line?,
        None => return Ok(()), // No input, just exit
    };
    let num_size: usize = num_size_line.trim().parse().expect("Invalid input for numSize");

    // Read the integers (like cin >> nums[i] in C++),
    // which could come from one or multiple lines until we have num_size values.
    let mut nums = Vec::with_capacity(num_size);
    while nums.len() < num_size {
        let line = match lines.next() {
            Some(l) => l?,
            None => break,
        };
        for part in line.split_whitespace() {
            if let Ok(value) = part.parse::<i32>() {
                nums.push(value);
                if nums.len() == num_size {
                    break;
                }
            }
        }
    }

    // Solve using the same approach
    let solution = Solution;
    let result = solution.max_good_number(&mut nums);
    // Output the result exactly like "cout << ... << endl;"
    println!("{}", result);

    Ok(())
}