use std::io::{self, Write};
use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};

struct Solution;

impl Solution {
    pub fn min_array_sum(nums: &mut Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
        let n = nums.len();
        nums.sort();

        // Find the boundaries for the three sections
        let m1 = nums.partition_point(|&x| x < k);
        let m2 = nums.partition_point(|&x| x < 2 * k - 1);

        let mut candidates = BTreeSet::new(); // To track indices of even numbers in the middle section
        let mut swap_cnt = 0;

        // Phase 1
        // Largest numbers, apply op1 then op2
        let mut i = n as i32 - 1;
        while i >= m2 as i32 && op1 > 0 {
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
        let mut j = m1 as i32;
        while j <= i && op2 > 0 {
            let index = j as usize;
            if k % 2 == 1 && nums[index] % 2 == 0 {
                // k is odd and nums[j] is even, mark as a candidate for swapping
                candidates.insert(index);
            }
            nums[index] -= k;
            op2 -= 1;
            j += 1;
        }

        // Phase 3
        // Apply op1 to numbers in the middle section not already affected by op2
        while i >= j && op1 > 0 {
            let index = i as usize;
            if k % 2 == 1 && nums[index] % 2 == 1 {
                // k is odd and nums[i] is odd, increase swap count
                swap_cnt += 1;
            }
            nums[index] = (nums[index] + 1) / 2;
            op1 -= 1;
            i -= 1;
        }

        // Phase 4
        // Sort remaining untouched numbers and apply op1 greedily
        let mut arr: VecDeque<(i32, usize)> = (0..j as usize)
            .map(|idx| (nums[idx], idx))
            .collect();
        arr.make_contiguous().sort_by(|a, b| b.cmp(a)); // Sort in descending order

        while op1 > 0 && !arr.is_empty() {
            let (num, idx) = arr.pop_back().unwrap();
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
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Read first line: n, k, op1, op2
    stdin.read_line(&mut input).unwrap();
    let mut params = input.trim().split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let k: i32 = params.next().unwrap().parse().unwrap();
    let op1: i32 = params.next().unwrap().parse().unwrap();
    let op2: i32 = params.next().unwrap().parse().unwrap();

    // Read second line: array of size n
    input.clear();
    stdin.read_line(&mut input).unwrap();
    let mut nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Create an instance of the Solution struct
    let result = Solution::min_array_sum(&mut nums, k, op1, op2);

    // Output the result
    writeln!(stdout, "{}", result).unwrap();
}