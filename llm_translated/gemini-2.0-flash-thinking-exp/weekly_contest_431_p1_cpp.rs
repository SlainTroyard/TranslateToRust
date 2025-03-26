use std::io;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_length(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        let mut m = 0;
        for &x in nums {
            m = max(m, x);
        }

        let mut fac: Vec<Vec<i32>> = vec![Vec::new(); (m + 1) as usize];
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
        let mut vis = vec![false; (m + 1) as usize];
        let mut j = 0;
        for i in 0..n {
            while j < n {
                let check = || -> bool {
                    for &p in &fac[nums[j] as usize] {
                        if vis[p as usize] {
                            return false;
                        }
                    }
                    return true;
                };

                if check() {
                    for &p in &fac[nums[j] as usize] {
                        vis[p as usize] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            ans = max(ans, (j - i) as i32);
            for &p in &fac[nums[i] as usize] {
                vis[p as usize] = false;
            }
        }
        return ans;
    }
}

fn main() {
    let mut num_size = String::new();
    io::stdin().read_line(&mut num_size).expect("Failed to read line");
    let num_size: usize = num_size.trim().parse().expect("Please type a number!");

    let mut nums_input = String::new();
    io::stdin().read_line(&mut nums_input).expect("Failed to read line");
    let nums: Vec<i32> = nums_input
        .split_whitespace()
        .map(|s| s.trim().parse().expect("Please type a number!"))
        .collect();

    let solution = Solution {};
    let result = solution.max_length(&nums);
    println!("{}", result);
}