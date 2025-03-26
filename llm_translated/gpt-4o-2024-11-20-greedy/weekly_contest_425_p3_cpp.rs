use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::{HashSet, BinaryHeap};
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn min_array_sum(nums: &mut Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
        let n = nums.len();
        nums.sort();

        // Find the boundaries for the three sections
        let m1 = nums.iter().position(|&x| x >= k).unwrap_or(n);
        let m2 = nums.iter().position(|&x| x >= 2 * k - 1).unwrap_or(n);

        let mut candidates = HashSet::new(); // To track indices of even numbers in the middle section
        let mut swap_cnt = 0;

        // Phase 1
        // Largest numbers, apply op1 then op2
        let mut i = n as isize - 1;
        while i >= m2 as isize && op1 > 0 {
            nums[i as usize] = (nums[i as usize] + 1) / 2;
            op1 -= 1;
            if op2 > 0 {
                nums[i as usize] -= k;
                op2 -= 1;
            }
            i -= 1;
        }

        // Phase 2
        // Apply op2 in the middle section, from left to right
        let mut j = m1 as isize;
        while j <= i && op2 > 0 {
            if k % 2 == 1 && nums[j as usize] % 2 == 0 {
                // k is odd and nums[j] is even, mark as a candidate for swapping
                candidates.insert(j as usize);
            }
            nums[j as usize] -= k;
            op2 -= 1;
            j += 1;
        }

        // Phase 3
        // Apply op1 to numbers in the middle section not already affected by op2
        while i >= j && op1 > 0 {
            if k % 2 == 1 && nums[i as usize] % 2 == 1 {
                // k is odd and nums[i] is odd, increase swap count
                swap_cnt += 1;
            }
            nums[i as usize] = (nums[i as usize] + 1) / 2;
            op1 -= 1;
            i -= 1;
        }

        // Phase 4
        // Sort remaining untouched numbers and apply op1 greedily
        let mut arr: Vec<(i32, usize)> = (0..j as usize)
            .map(|idx| (nums[idx], idx))
            .collect();
        arr.sort_by(|a, b| b.0.cmp(&a.0)); // Sort in descending order

        while op1 > 0 && !arr.is_empty() {
            let (num, idx) = arr.pop().unwrap();
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

    // Read the first line of input
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let k: i32 = first_iter.next().unwrap().parse().unwrap();
    let op1: i32 = first_iter.next().unwrap().parse().unwrap();
    let op2: i32 = first_iter.next().unwrap().parse().unwrap();

    // Read the second line of input
    let second_line = lines.next().unwrap().unwrap();
    let mut nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Create an instance of the Solution struct
    let result = Solution::min_array_sum(&mut nums, k, op1, op2);

    // Output the result
    println!("{}", result);
}