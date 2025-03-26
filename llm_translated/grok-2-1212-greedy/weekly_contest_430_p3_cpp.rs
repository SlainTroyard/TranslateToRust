use std::io::{self, BufRead};
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn number_of_subsequences(nums: &Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf = HashMap::new();

        for i in 4..n - 2 {
            let c = nums[i];
            for j in i + 2..n {
                let d = nums[j];
                let g = Self::gcd(c, d);
                let key = (d / g) << 16 | (c / g);
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans = 0;
        for i in 2..n - 4 {
            let b = nums[i];
            for j in 0..i - 1 {
                let a = nums[j];
                let g = Self::gcd(a, b);
                let key = (a / g) << 16 | (b / g);
                ans += suf.get(&key).unwrap_or(&0);
            }
            let c = nums[i + 2];
            for j in i + 4..n {
                let d = nums[j];
                let g = Self::gcd(c, d);
                let key = (d / g) << 16 | (c / g);
                *suf.entry(key).or_insert(0) -= 1;
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Input the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the number_of_subsequences function
    let result = Solution::number_of_subsequences(&nums);

    // Output the result
    println!("{}", result);

    Ok(())
}