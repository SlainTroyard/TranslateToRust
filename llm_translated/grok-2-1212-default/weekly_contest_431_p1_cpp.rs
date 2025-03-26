use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_length(nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        let m = nums.iter().max().unwrap_or(&0);

        // Initialize factor vectors
        let mut fac = vec![vec![]; (m + 1) as usize];
        for i in 2..=m {
            if fac[i as usize].is_empty() {
                for j in (i..=m).step_by(i as usize) {
                    fac[j as usize].push(i);
                }
            }
        }

        let mut ans = 2;
        let mut vis = vec![false; (m + 1) as usize];
        let mut j = 0;

        for i in 0..n {
            while j < n {
                let check = || {
                    for &p in &fac[nums[j] as usize] {
                        if vis[p as usize] {
                            return false;
                        }
                    }
                    true
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
            ans = ans.max((j - i) as i32);
            for &p in &fac[nums[i] as usize] {
                vis[p as usize] = false;
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the vector
    let num_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the vector elements
    let nums: Vec<i32> = lines
        .take(num_size)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::max_length(&nums);
    println!("{}", result);

    Ok(())
}