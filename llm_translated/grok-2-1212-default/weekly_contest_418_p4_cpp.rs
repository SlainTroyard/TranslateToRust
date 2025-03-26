use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn gcd_values(nums: &Vec<i32>, queries: &Vec<i64>) -> Vec<i32> {
        let mx = *nums.iter().max().unwrap() as usize;
        let mut cnt_x = vec![0; mx + 1];
        for &x in nums {
            cnt_x[x as usize] += 1;
        }

        let mut cnt_gcd = vec![0; mx + 1];
        for i in (1..=mx).rev() {
            let mut c = 0;
            for j in (i..=mx).step_by(i) {
                c += cnt_x[j];
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += (c * (c - 1) / 2) as i64;
        }
        for i in 1..=mx {
            cnt_gcd[i] += cnt_gcd[i - 1];
        }

        let ans: Vec<i32> = queries
            .iter()
            .map(|&query| {
                cnt_gcd
                    .iter()
                    .position(|&x| x > query)
                    .map_or(mx as i32 + 1, |pos| pos as i32)
            })
            .collect();
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and nums
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read q and queries
    let q: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let queries: Vec<i64> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Solve the problem
    let solution = Solution;
    let ans = solution.gcd_values(&nums, &queries);

    // Print the result
    for x in ans {
        print!("{} ", x);
    }
    println!();

    Ok(())
}