use std::io;
use std::io::Read;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let op1: i32 = parts.next().unwrap().parse().unwrap();
    let op2: i32 = parts.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap();
    let nums: Vec<i32> = second_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = Solution::new().min_array_sum(nums, k, op1, op2);
    println!("{}", result);
}

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    fn min_array_sum(&self, mut nums: Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
        let n = nums.len();
        nums.sort();

        // Find the boundaries for the three sections
        let m1 = match nums.binary_search(&k) {
            Ok(i) => i,
            Err(i) => i,
        };
        let m2 = match nums.binary_search(&(2 * k - 1)) {
            Ok(i) => i,
            Err(i) => i,
        };

        let mut candidates: std::collections::HashSet<usize> = std::collections::HashSet::new(); // To track indices of even numbers in the middle section
        let mut swap_cnt = 0;

        // Phase 1
        // Largest numbers, apply op1 then op2
        let mut i = n as i32 - 1;
        while i >= m2 as i32 && op1 > 0 {
            let i_usize = i as usize;
            nums[i_usize] = (nums[i_usize] + 1) / 2;
            op1 -= 1;
            if op2 > 0 {
                nums[i_usize] -= k;
                op2 -= 1;
            }
            i -= 1;
        }

        // Phase 2
        // Apply op2 in the middle section, from left to right
        let mut j = m1;
        while j <= i as usize && op2 > 0 {
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
        while i >= j as i32 && op1 > 0 {
            let i_usize = i as usize;
            if k % 2 == 1 && nums[i_usize] % 2 == 1 {
                // k is odd and nums[i] is odd, increase swap count
                swap_cnt += 1;
            }
            nums[i_usize] = (nums[i_usize] + 1) / 2;
            op1 -= 1;
            i -= 1;
        }

        // Phase 4
        // Sort remaining untouched numbers and apply op1 greedily
        let mut arr: Vec<(i32, usize)> = Vec::new();
        for idx in 0..j {
            arr.push((nums[idx], idx));
        }
        arr.sort(); // Sort in ascending order

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