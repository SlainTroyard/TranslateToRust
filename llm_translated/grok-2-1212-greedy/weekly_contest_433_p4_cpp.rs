use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn min_max_subarray_sum(nums: &mut Vec<i32>, k: i32) -> i64 {
        let count = |m: i32| -> i64 {
            if m > k {
                (m as i64 * 2 - k as i64 + 1) * k as i64 / 2
            } else {
                (m as i64 + 1) * m as i64 / 2
            }
        };

        let sum_subarray_mins = || -> i64 {
            let mut res = 0;
            let mut st: Vec<usize> = vec![usize::MAX];
            for r in 0..nums.len() {
                while st.len() > 1 && nums[*st.last().unwrap()] >= nums[r] {
                    let i = *st.last().unwrap();
                    st.pop();
                    let l = *st.last().unwrap();
                    let cnt = count((r - l - 1) as i32) - count((i - l - 1) as i32) - count((r - i - 1) as i32);
                    res += nums[i] as i64 * cnt;
                }
                st.push(r);
            }
            res
        };

        nums.push(i32::MIN / 2);
        let mut ans = sum_subarray_mins();
        for x in nums.iter_mut() {
            *x = -*x;
        }
        *nums.last_mut().unwrap() *= -1;
        ans -= sum_subarray_mins();
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read nums
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Calculate and print result
    let mut nums = nums;
    let result = Solution::min_max_subarray_sum(&mut nums, k);
    println!("{}", result);

    Ok(())
}