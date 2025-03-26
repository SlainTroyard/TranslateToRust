use std::io;
use std::io::prelude::*;

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

        let mut ans = Vec::with_capacity(queries.len());
        for &query in &queries {
            let pos = prefix_sum.iter().position(|&x| x > query).unwrap_or(mx + 1);
            ans.push(pos as i32);
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let q = lines.next().unwrap().parse::<usize>().unwrap();
    let queries: Vec<i64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let solution = Solution {};
    let ans = solution.gcd_values(nums, queries);

    for x in ans {
        print!("{} ", x);
    }
    println!();

    Ok(())
}