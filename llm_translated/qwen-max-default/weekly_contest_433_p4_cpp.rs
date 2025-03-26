use std::io::{self, BufRead, Write};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: usize = iter.next().unwrap().parse().expect("Failed to parse k");

    let mut nums: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        let num: i64 = buffer.trim().parse().expect("Failed to parse number");
        nums.push(num);
    }

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.min_max_subarray_sum(nums, k);

    // Write the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn min_max_subarray_sum(nums: Vec<i64>, k: usize) -> i64 {
        let count = |m: usize| -> i64 {
            if m > k {
                (m as i64 * 2 - k as i64 + 1) * k as i64 / 2
            } else {
                (m as i64 + 1) * m as i64 / 2
            }
        };

        let sum_subarray_mins = || -> i64 {
            let mut res = 0;
            let mut st = vec![-1];
            for r in 0..nums.len() {
                while st.len() > 1 && nums[st[st.len() - 1]] >= nums[r] {
                    let i = st.pop().unwrap();
                    let l = *st.last().unwrap();
                    let cnt = count(r - l - 1) - count(i - l - 1) - count(r - i - 1);
                    res += nums[i] * cnt;
                }
                st.push(r as i64);
            }
            res
        };

        let mut nums = nums;
        nums.push(i64::MIN / 2);
        let ans = sum_subarray_mins();
        for x in &mut nums {
            *x = -*x;
        }
        nums[nums.len() - 1] *= -1;
        ans - sum_subarray_mins()
    }
}