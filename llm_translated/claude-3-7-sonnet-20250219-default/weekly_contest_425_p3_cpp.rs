use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    pub fn min_array_sum(mut nums: Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
        let n = nums.len();
        nums.sort_unstable();

        // Find the boundaries for the three sections
        let m1 = nums.partition_point(|&x| x < k);
        let m2 = nums.partition_point(|&x| x < 2 * k - 1);

        let mut candidates: HashSet<usize> = HashSet::new(); // To track indices of even numbers in the middle section
        let mut swap_cnt = 0;

        // Phase 1
        // Largest numbers, apply op1 then op2
        let mut i = n as i32 - 1;
        let mut op1_remaining = op1;
        let mut op2_remaining = op2;
        
        while i >= m2 as i32 && op1_remaining > 0 {
            let idx = i as usize;
            nums[idx] = (nums[idx] + 1) / 2;
            op1_remaining -= 1;
            if op2_remaining > 0 {
                nums[idx] -= k;
                op2_remaining -= 1;
            }
            i -= 1;
        }

        // Update the remaining operations
        op1 -= (op1 - op1_remaining);
        op2 -= (op2 - op2_remaining);

        // Phase 2
        // Apply op2 in the middle section, from left to right
        let mut j = m1;
        let mut op2_remaining = op2;
        
        while j <= i as usize && op2_remaining > 0 {
            if k % 2 == 1 && nums[j] % 2 == 0 {
                // k is odd and nums[j] is even, mark as a candidate for swapping
                candidates.insert(j);
            }
            nums[j] -= k;
            op2_remaining -= 1;
            j += 1;
        }

        // Update the remaining operations
        op2 -= (op2 - op2_remaining);

        // Phase 3
        // Apply op1 to numbers in the middle section not already affected by op2
        let mut op1_remaining = op1;
        
        while i >= j as i32 && op1_remaining > 0 {
            let idx = i as usize;
            if k % 2 == 1 && nums[idx] % 2 == 1 {
                // k is odd and nums[i] is odd, increase swap count
                swap_cnt += 1;
            }
            nums[idx] = (nums[idx] + 1) / 2;
            op1_remaining -= 1;
            i -= 1;
        }

        // Update the remaining operations
        op1 -= (op1 - op1_remaining);

        // Phase 4
        // Sort remaining untouched numbers and apply op1 greedily
        let mut arr: Vec<(i32, usize)> = Vec::new();
        for idx in 0..j {
            arr.push((nums[idx], idx));
        }
        arr.sort_unstable(); // Sort in ascending order (smallest first)

        while op1 > 0 && !arr.is_empty() {
            let (num, idx) = arr.pop().unwrap(); // Take the largest number
            nums[idx] = (num + 1) / 2;
            op1 -= 1;

            if candidates.contains(&idx) && swap_cnt > 0 {
                // Handle the swap case
                swap_cnt -= 1;
                nums[idx] -= 1;
            }
        }

        // Return the sum of the modified nums
        nums.iter().sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input parameters
    let first_line = lines.next().unwrap().unwrap();
    let params: Vec<i32> = first_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    let n = params[0];
    let k = params[1];
    let op1 = params[2];
    let op2 = params[3];
    
    // Read the nums array
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    // Create an instance of the Solution struct
    let solution = Solution {};
    
    // Call the min_array_sum function and output the result
    let result = solution.min_array_sum(nums, k, op1, op2);
    println!("{}", result);
}