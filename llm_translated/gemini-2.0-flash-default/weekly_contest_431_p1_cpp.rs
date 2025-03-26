use std::io;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_length(nums: Vec<int>) -> int {
        let n = nums.len();
        let mut m = 0;
        for &x in &nums {
            m = max(m, x);
        }

        let mut fac: Vec<Vec<int>> = vec![Vec::new(); (m + 1) as usize];
        for i in 2..=m {
            if fac[i as usize].is_empty() {
                let mut j = i;
                while j <= m {
                    fac[j as usize].push(i);
                    j += i;
                }
            }
        }

        let mut ans = 2;
        let mut vis: Vec<bool> = vec![false; (m + 1) as usize];
        let mut j = 0;
        for i in 0..n {
            while j < n {
                let mut check = true;
                for &p in &fac[nums[j] as usize] {
                    if vis[p as usize] {
                        check = false;
                        break;
                    }
                }
                if check {
                    for &p in &fac[nums[j] as usize] {
                        vis[p as usize] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            ans = max(ans, (j - i) as int);
            for &p in &fac[nums[i] as usize] {
                vis[p as usize] = false;
            }
        }
        ans
    }
}

fn main() {
    let mut num_size_str = String::new();
    io::stdin().read_line(&mut num_size_str).expect("Failed to read line");
    let num_size: int = num_size_str.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<int> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let solution = Solution {};
    let result = solution.max_length(nums);
    println!("{}", result);
}