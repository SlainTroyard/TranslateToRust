use std::io;
use std::cmp;

struct Solution {}

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let mx = *nums.iter().max().unwrap() as usize;
        let mut cnt_x = vec![0; mx + 1];
        for &x in &nums {
            cnt_x[x as usize] += 1;
        }

        let mut cnt_gcd = vec![0i64; mx + 1];
        for i in (1..=mx).rev() {
            let mut c = 0;
            for j in (i..=mx).step_by(i) {
                c += cnt_x[j];
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += (c as i64) * (c as i64 - 1) / 2;
        }

        let mut prefix_sum = vec![0i64; mx + 1];
        prefix_sum[0] = cnt_gcd[0];
        for i in 1..=mx {
            prefix_sum[i] = prefix_sum[i - 1] + cnt_gcd[i];
        }

        let mut ans = vec![0; queries.len()];
        for i in 0..queries.len() {
            let mut left = 0;
            let mut right = mx as i32;
            let mut result = mx as i32 + 1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if mid as usize <= mx && prefix_sum[mid as usize] > queries[i] {
                    result = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            ans[i] = result;
        }

        ans
    }
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut q_str = String::new();
    io::stdin().read_line(&mut q_str).unwrap();
    let q: i32 = q_str.trim().parse().unwrap();

    let mut queries: Vec<i64> = Vec::new();
    for _ in 0..q {
        let mut query_str = String::new();
        io::stdin().read_line(&mut query_str).unwrap();
        let query: i64 = query_str.trim().parse().unwrap();
        queries.push(query);
    }

    let solution = Solution {};
    let ans = solution.gcd_values(nums, queries);

    for x in ans {
        print!("{} ", x);
    }
    println!();
}