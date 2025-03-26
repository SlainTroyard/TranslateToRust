use std::io::{self, BufRead, Write};
use std::collections::BTreeSet;

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line for n, k, op1, op2
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let op1: i32 = iter.next().unwrap().parse().unwrap();
    let op2: i32 = iter.next().unwrap().parse().unwrap();

    // Parse the second line for the nums array
    let nums_line = lines.next().unwrap().unwrap();
    let mut nums: Vec<i32> = nums_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Create an instance of the Solution struct and call the min_array_sum function
    let solution = Solution;
    let result = solution.min_array_sum(&mut nums, k, op1, op2);

    // Output the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn min_array_sum(nums: &mut Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let n = nums.len();
        nums.sort_unstable();

        // Find the boundaries for the three sections
        let m1 = nums.partition_point(|&x| x < k);
        let m2 = nums.partition_point(|&x| x < 2 * k - 1);

        let mut candidates = BTreeSet::new(); // To track indices of even numbers in the middle section
        let mut swap_cnt = 0;

        // Phase 1
        // Largest numbers, apply op1 then op2
        let mut i = n - 1;
        while i >= m2 && op1 > 0 {
            nums[i] = (nums[i] + 1) / 2;
            if op2 > 0 {
                nums[i] -= k;
                op2 -= 1;
            }
            op1 -= 1;
            i -= 1;
        }

        // Phase 2
        // Apply op2 in the middle section, from left to right
        let mut j = m1;
        while j <= i && op2 > 0 {
            if k % 2 == 1 && nums[j] % 2 == 0 {
                // k is odd and nums[j] is even, mark as a candidate for swapping
                candidates.insert(j);
            }
            nums[j] -= k;
            op2 -= 1;
            j += 1;
        }

        // Phase 3
        // Apply op1 to numbers in the middle section not already affected by op2
        while i >= j && op1 > 0 {
            if k % 2 == 1 && nums[i] % 2 == 1 {
                // k is odd and nums[i] is odd, increase swap count
                swap_cnt += 1;
            }
            nums[i] = (nums[i] + 1) / 2;
            op1 -= 1;
            i -= 1;
        }

        // Phase 4
        // Sort remaining untouched numbers and apply op1 greedily
        let mut arr: Vec<(i32, usize)> = nums.iter().enumerate()
            .filter(|&(idx, _)| idx < j)
            .map(|(idx, &num)| (num, idx))
            .collect();
        arr.sort_unstable_by_key(|&(num, _)| num); // Sort in ascending order

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