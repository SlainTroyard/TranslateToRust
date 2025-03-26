use std::io;

pub struct Solution;

impl Solution {
    pub fn min_max_subarray_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        // Closure to compute the count as per the formula
        let count = |m: i32| -> i64 {
            if m > k {
                let numerator = (2 * m - k + 1) as i64 * k as i64;
                numerator / 2
            } else {
                (m as i64) * (m as i64 + 1) / 2
            }
        };

        // Closure to compute the sum of subarray minimums using a stack-based approach
        let sum_subarray_mins = || {
            let mut stack = vec![-1i32]; // Stack to track indices with a sentinel
            let mut res = 0i64;
            for r in 0..nums.len() {
                // Pop elements from the stack until the top element's value is smaller than current
                while stack.len() > 1 && nums[stack[stack.len() - 1] as usize] >= nums[r] {
                    let i = stack.pop().unwrap();
                    let l = stack[stack.len() - 1];
                    let m1 = (r as i32 - l as i32 - 1) as i32;
                    let m2 = (i as i32 - l as i32 - 1) as i32;
                    let m3 = (r as i32 - i as i32 - 1) as i32;
                    let cnt = count(m1) - count(m2) - count(m3);
                    res += nums[i as usize] as i64 * cnt;
                }
                stack.push(r as i32);
            }
            res
        };

        // Append the sentinel value (INT_MIN / 2) to handle edge cases
        nums.push(i32::MIN / 2);
        let ans = sum_subarray_mins();
        
        // Invert all elements except the sentinel, then invert the sentinel back
        for x in nums.iter_mut() {
            *x = -*x;
        }
        *nums.last_mut().unwrap() *= -1;
        
        ans - sum_subarray_mins()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    let mut nums = Vec::new();
    for line in lines {
        for token in line.split_whitespace() {
            nums.push(token.parse().unwrap());
        }
    }
    assert_eq!(nums.len(), n);
    
    let solution = Solution {};
    println!("{}", solution.min_max_subarray_sum(nums, k));
}