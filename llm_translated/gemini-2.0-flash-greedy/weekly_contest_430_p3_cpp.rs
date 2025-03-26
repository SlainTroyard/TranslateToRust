use std::collections::HashMap;
use std::io;
use std::str::FromStr;

// Function to calculate the greatest common divisor (GCD)
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution {}

impl Solution {
    pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf: HashMap<i32, i32> = HashMap::new();

        // Precompute the suffix counts
        for i in 4..n - 2 {
            let c = nums[i];
            for j in i + 2..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = (d / g) << 16 | (c / g);
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans: i64 = 0;

        for i in 2..n - 4 {
            let b = nums[i];
            for j in 0..i - 1 {
                let a = nums[j];
                let g = gcd(a, b);
                let key = (a / g) << 16 | (b / g);
                ans += *suf.get(&key).unwrap_or(&0) as i64;
            }

            let c = nums[i + 2];
            for j in i + 4..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = (d / g) << 16 | (c / g);
                if let Some(count) = suf.get_mut(&key) {
                    *count -= 1;
                }
            }
        }

        ans
    }
}

fn main() {
    let mut n_str = String::new();
    io::stdin()
        .read_line(&mut n_str)
        .expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin()
        .read_line(&mut nums_str)
        .expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let solution = Solution {};
    let result = solution.number_of_subsequences(nums);
    println!("{}", result);
}