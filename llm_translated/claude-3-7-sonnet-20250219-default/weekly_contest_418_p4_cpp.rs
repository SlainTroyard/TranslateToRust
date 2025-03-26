use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let mx = *nums.iter().max().unwrap();
        let mut cnt_x = vec![0; mx as usize + 1];
        for x in nums {
            cnt_x[x as usize] += 1;
        }

        let mut cnt_gcd = vec![0i64; mx as usize + 1];
        for i in (1..=mx as usize).rev() {
            let mut c = 0;
            for j in (i..=mx as usize).step_by(i) {
                c += cnt_x[j];
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += (c as i64) * (c as i64 - 1) / 2;
        }
        
        // Calculate prefix sum (equivalent to partial_sum in C++)
        for i in 1..=mx as usize {
            cnt_gcd[i] += cnt_gcd[i - 1];
        }

        let mut ans = vec![0; queries.len()];
        for i in 0..queries.len() {
            // upper_bound equivalent in Rust
            ans[i] = cnt_gcd.partition_point(|&x| x <= queries[i]) as i32;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of elements in nums
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums array
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read number of queries
    let q: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read queries array
    let queries_line = lines.next().unwrap()?;
    let queries: Vec<i64> = queries_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Solve and print results
    let ans = Solution::gcd_values(nums, queries);
    
    // Print answers with spaces between them
    let output: String = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);

    Ok(())
}