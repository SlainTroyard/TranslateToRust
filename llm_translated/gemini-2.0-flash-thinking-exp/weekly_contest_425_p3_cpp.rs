use std::io;
use std::vec::Vec;
use std::cmp::min;
use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn min_array_sum(nums: &mut Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let n = nums.len();
        nums.sort();

        // Find the boundaries for the three sections
        let m1 = nums.partition_point(|&x| x < k);
        let m2 = nums.partition_point(|&x| x < 2 * k - 1);

        let mut candidates: BTreeSet<usize> = BTreeSet::new(); // To track indices of even numbers in the middle section
        let mut swap_cnt = 0;
        let mut current_op1 = op1;
        let mut current_op2 = op2;

        // Phase 1
        // Largest numbers, apply op1 then op2
        let mut i = n as i32 - 1;
        while i >= m2 as i32 && current_op1 > 0 {
            let i_usize = i as usize;
            nums[i_usize] = (nums[i_usize] + 1) / 2;
            current_op1 -= 1;
            if current_op2 > 0 {
                nums[i_usize] -= k;
                current_op2 -= 1;
            }
            i -= 1;
        }

        // Phase 2
        // Apply op2 in the middle section, from left to right
        let mut j = m1;
        while j <= i as usize && current_op2 > 0 {
            if k % 2 == 1 && nums[j] % 2 == 0 {
                // k is odd and nums[j] is even, mark as a candidate for swapping
                candidates.insert(j);
            }
            nums[j] -= k;
            current_op2 -= 1;
            j += 1;
        }

        // Phase 3
        // Apply op1 to numbers in the middle section not already affected by op2
        while i as usize >= j && current_op1 > 0 {
            let i_usize = i as usize;
            if k % 2 == 1 && nums[i_usize] % 2 == 1 {
                // k is odd and nums[i] is odd, increase swap count
                swap_cnt += 1;
            }
            nums[i_usize] = (nums[i_usize] + 1) / 2;
            current_op1 -= 1;
            i -= 1;
        }

        // Phase 4
        // Sort remaining untouched numbers and apply op1 greedily
        let mut arr: Vec<(i32, usize)> = Vec::new();
        for idx in 0..j {
            arr.push((nums[idx], idx));
        }
        arr.sort(); // Sort in ascending order by default, which is what we need to pop_back from the end

        while current_op1 > 0 && !arr.is_empty() {
            if let Some((num, idx)) = arr.pop() {
                nums[idx] = (num + 1) / 2;
                current_op1 -= 1;

                if candidates.contains(&idx) && swap_cnt > 0 {
                    // Handle the swap case
                    swap_cnt -= 1;
                    nums[idx] -= 1;
                }
            }
        }

        // Return the sum of the modified nums
        nums.iter().sum()
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let parts: Vec<i32> = input_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = parts[0] as usize;
    let k = parts[1];
    let op1 = parts[2];
    let op2 = parts[3];

    let mut input_line_nums = String::new();
    io::stdin().read_line(&mut input_line_nums).expect("Failed to read line");
    let mut nums: Vec<i32> = input_line_nums
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution {};
    let result = solution.min_array_sum(&mut nums, k, op1, op2);
    println!("{}", result);
}