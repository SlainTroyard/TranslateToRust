use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn min_array_sum(nums: &mut Vec<i32>, k: i32, op1: i32, mut op2: i32) -> i32 {
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
        let mut op1_left = op1;
        while i >= m2 && op1_left > 0 {
            nums[i] = (nums[i] + 1) / 2;
            op1_left -= 1;
            if op2 > 0 {
                nums[i] -= k;
                op2 -= 1;
            }
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
        while i >= j && op1_left > 0 {
            if k % 2 == 1 && nums[i] % 2 == 1 {
                // k is odd and nums[i] is odd, increase swap count
                swap_cnt += 1;
            }
            nums[i] = (nums[i] + 1) / 2;
            op1_left -= 1;
            i -= 1;
        }

        // Phase 4
        // Sort remaining untouched numbers and apply op1 greedily
        let mut arr: Vec<(i32, usize)> = nums.iter().take(j).enumerate().map(|(idx, &num)| (num, idx)).collect();
        arr.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // Sort in descending order

        while op1_left > 0 && !arr.is_empty() {
            let (num, idx) = arr.pop().unwrap();
            nums[idx] = (num + 1) / 2;
            op1_left -= 1;

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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input reading
    let first_line: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = first_line[0] as usize;
    let k = first_line[1];
    let op1 = first_line[2];
    let op2 = first_line[3];

    let mut nums: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(n)
        .collect();

    // Call the min_array_sum function and output the result
    let result = Solution::min_array_sum(&mut nums, k, op1, op2);
    println!("{}", result);

    Ok(())
}