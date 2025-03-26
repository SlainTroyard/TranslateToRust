use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn min_max_subarray_sum(nums: &mut Vec<i32>, k: i32) -> i64 {
        // Push a sentinel value to handle the last elements
        nums.push(i32::MIN / 2);
        let ans = Self::sum_subarray_mins(nums, k);

        // Invert elements except the last one (which was the sentinel)
        for x in nums.iter_mut().take(nums.len() - 1) {
            *x = -*x;
        }
        if let Some(last) = nums.last_mut() {
            *last = -*last; // Revert the sentinel to original value
        }

        let ans2 = Self::sum_subarray_mins(nums, k);
        ans - ans2
    }

    // Helper function to calculate the sum using monotonic stack
    fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
        let count = |m: i32| -> i64 {
            if m > k {
                (m as i64 * 2 - k as i64 + 1) * k as i64 / 2
            } else {
                (m as i64 + 1) * m as i64 / 2
            }
        };

        let mut res = 0i64;
        let mut stack = vec![-1i32]; // Stack to keep indices

        for r in 0..nums.len() {
            let r_i32 = r as i32;

            // Maintain monotonically increasing stack
            while stack.len() > 1 && nums[*stack.last().unwrap() as usize] >= nums[r] {
                let i = stack.pop().unwrap();
                let l = *stack.last().unwrap();

                // Calculate contribution using count function
                let cnt = count(r_i32 - l - 1) 
                    - count(i - l - 1) 
                    - count(r_i32 - i - 1);
                res += nums[i as usize] as i64 * cnt;
            }
            stack.push(r_i32);
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(tokens.next().unwrap().parse().unwrap());
    }

    println!("{}", Solution::min_max_subarray_sum(&mut nums, k));
}