// Weekly Contest 425 Problem 3 (Rust Translation)
// This program reads input from stdin, applies the algorithm from the C++ solution,
// and writes the result to stdout. The logic is preserved exactly.

use std::io::{self, BufRead};
use std::collections::HashSet;

struct Solution;

impl Solution {
    fn min_array_sum(nums: &mut [i32], k: i32, mut op1: i32, mut op2: i32) -> i32 {
        let n = nums.len();
        // Sort the numbers
        nums.sort();

        // Find the boundaries for the three sections using binary search
        let m1 = match nums.binary_search(&k) {
            Ok(pos) | Err(pos) => pos
        };
        let m2 = match nums.binary_search(&(2 * k - 1)) {
            Ok(pos) | Err(pos) => pos
        };

        // This set will hold indices of items eligible for a later swap
        let mut candidates: HashSet<usize> = HashSet::new();

        // Tracks how many swaps can be used in Phase 4
        let mut swap_cnt = 0;

        // Phase 1: Largest numbers, apply op1 then op2
        let mut i = if n > 0 { n - 1 } else { 0 };
        while op1 > 0 {
            if i < m2 {
                break;
            }
            // Apply op1
            nums[i] = (nums[i] + 1) / 2;
            op1 -= 1;

            // Apply op2
            if op2 > 0 {
                nums[i] -= k;
                op2 -= 1;
            }

            // If we've reached the beginning, we must break to avoid underflow
            if i == 0 {
                break;
            }
            i -= 1;
        }

        // Phase 2: Apply op2 in the middle section, from left to right
        let mut j = m1;
        while j <= i && j < n && op2 > 0 {
            // If k is odd and nums[j] is even, mark for a possible future swap
            if k % 2 == 1 && nums[j] % 2 == 0 {
                candidates.insert(j);
            }
            nums[j] -= k;
            op2 -= 1;
            j += 1;
        }

        // Phase 3: Apply remaining op1 in the middle section, from right to left
        while op1 > 0 && i >= j {
            // If k is odd and nums[i] is odd, increment swap count
            if k % 2 == 1 && nums[i] % 2 == 1 {
                swap_cnt += 1;
            }
            nums[i] = (nums[i] + 1) / 2;
            op1 -= 1;

            if i == 0 {
                break;
            }
            i -= 1;
        }

        // Phase 4: Sort the left (0..j) section and apply op1 greedily from largest to smallest
        let mut arr: Vec<(i32, usize)> = Vec::new();
        for idx in 0..j {
            arr.push((nums[idx], idx));
        }
        // Sort in ascending order, so that popping from the back yields the largest element
        arr.sort_by_key(|&(val, _)| val);

        while op1 > 0 && !arr.is_empty() {
            let (num, idx) = arr.pop().unwrap();
            nums[idx] = (num + 1) / 2;
            op1 -= 1;

            // If this index was marked a candidate and we still have swaps left
            if candidates.contains(&idx) && swap_cnt > 0 {
                swap_cnt -= 1;
                nums[idx] -= 1;
            }
        }

        // Compute the sum of the modified array
        nums.iter().sum()
    }
}

fn main() {
    // Read the first line: n, k, op1, op2
    let stdin = io::stdin();
    let mut input_line = String::new();
    stdin.lock().read_line(&mut input_line).unwrap();
    let mut parts = input_line.split_whitespace();

    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let op1: i32 = parts.next().unwrap().parse().unwrap();
    let op2: i32 = parts.next().unwrap().parse().unwrap();

    // Read the next line containing the array
    input_line.clear();
    stdin.lock().read_line(&mut input_line).unwrap();
    let mut nums_iter = input_line.split_whitespace();
    let mut nums = Vec::with_capacity(n);

    for _ in 0..n {
        let val: i32 = nums_iter.next().unwrap().parse().unwrap();
        nums.push(val);
    }

    // Create an instance of the Solution struct and call the min_array_sum method
    let result = Solution::min_array_sum(&mut nums, k, op1, op2);
    
    // Output the result
    println!("{}", result);
}