use std::collections::HashSet;
use std::io;

struct Solution;

impl Solution {
    pub fn min_array_sum(mut nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let m1 = nums.partition_point(|&x| x < k);
        let m2 = nums.partition_point(|&x| x < 2 * k - 1);

        let mut op1_remaining = op1;
        let mut op2_remaining = op2;
        let mut i = n - 1;

        // Phase 1: Apply op1 and op2 to the largest elements
        while i >= m2 && op1_remaining > 0 {
            nums[i] = (nums[i] + 1) / 2;
            op1_remaining -= 1;
            if op2_remaining > 0 {
                nums[i] -= k;
                op2_remaining -= 1;
            }
            i -= 1;
        }

        let mut j = m1;
        let mut candidates = HashSet::new();

        // Phase 2: Apply op2 to the middle section
        while j <= i && op2_remaining > 0 {
            nums[j] -= k;
            op2_remaining -= 1;
            if k % 2 == 1 && nums[j] % 2 == 0 {
                candidates.insert(j);
            }
            j += 1;
        }

        let mut swap_cnt = 0;

        // Phase 3: Apply op1 to the remaining middle section
        while i >= j && op1_remaining > 0 {
            nums[i] = (nums[i] + 1) / 2;
            op1_remaining -= 1;
            if k % 2 == 1 && nums[i] % 2 == 1 {
                swap_cnt += 1;
            }
            i -= 1;
        }

        // Phase 4: Apply op1 to the remaining elements
        let mut arr = Vec::new();
        for idx in 0..j {
            arr.push((nums[idx], idx));
        }
        arr.sort_by(|a, b| b.0.cmp(&a.0));

        while op1_remaining > 0 && !arr.is_empty() {
            let (num, idx) = arr.pop().unwrap();
            nums[idx] = (num + 1) / 2;
            op1_remaining -= 1;
            if candidates.contains(&idx) && swap_cnt > 0 {
                nums[idx] -= 1;
                swap_cnt -= 1;
            }
        }

        nums.iter().sum()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid input");
    let k: i32 = parts.next().unwrap().parse().expect("Invalid input");
    let op1: i32 = parts.next().unwrap().parse().expect("Invalid input");
    let op2: i32 = parts.next().unwrap().parse().expect("Invalid input");

    let mut nums = Vec::new();
    for _ in 0..n {
        let num: i32 = parts.next().unwrap().parse().expect("Invalid input");
        nums.push(num);
    }

    let solution = Solution;
    let result = solution.min_array_sum(nums, k, op1, op2);
    println!("{}", result);
}