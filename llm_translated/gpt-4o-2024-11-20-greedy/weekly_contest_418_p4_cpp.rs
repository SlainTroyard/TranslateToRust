use std::io::{self, BufRead};
use std::cmp;
use std::collections::VecDeque;

struct Solution;

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
            cnt_gcd[i] += (c as i64 * (c as i64 - 1)) / 2;
        }

        // Perform partial sum
        for i in 1..=mx {
            cnt_gcd[i] += cnt_gcd[i - 1];
        }

        let mut ans = Vec::with_capacity(queries.len());
        for &query in &queries {
            let pos = cnt_gcd.iter().position(|&x| x > query).unwrap_or(mx + 1);
            ans.push(pos as i32);
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in nums
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the nums array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the number of queries
    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the queries array
    let queries: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let solution = Solution;
    let ans = solution.gcd_values(nums, queries);

    // Print the result
    for x in ans {
        print!("{} ", x);
    }
    println!();
}