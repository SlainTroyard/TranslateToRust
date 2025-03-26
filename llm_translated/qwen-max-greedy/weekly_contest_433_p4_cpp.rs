use std::io::{self, BufRead, Write};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: i32 = iter.next().unwrap().parse().expect("Failed to parse k");

    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        nums.push(buffer.trim().parse().expect("Failed to parse num"));
    }

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.min_max_subarray_sum(nums, k);

    // Write the result to stdout
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    writeln!(stdout_lock, "{}", result).expect("Failed to write result");
}

struct Solution;

impl Solution {
    pub fn min_max_subarray_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        let count = |m: i32| -> i64 {
            if m > k {
                (m as i64 * 2 - k as i64 + 1) * k as i64 / 2
            } else {
                (m as i64 + 1) * m as i64 / 2
            }
        };

        let sum_subarray_mins = || -> i64 {
            let mut res = 0i64;
            let mut st = vec![-1];
            for r in 0..nums.len() {
                while st.len() > 1 && nums[st[st.len() - 1]] >= nums[r] {
                    let i = st.pop().unwrap();
                    let l = st[st.len() - 1];
                    let cnt = count((r - l - 1) as i32) - count((i - l - 1) as i32) - count((r - i - 1) as i32);
                    res += nums[i] as i64 * cnt;
                }
                st.push(r);
            }
            res
        };

        nums.push(i32::MIN / 2);
        let ans = sum_subarray_mins();
        for x in &mut nums {
            *x = -*x;
        }
        nums[nums.len() - 1] *= -1;
        let ans = ans - sum_subarray_mins();
        ans
    }
}