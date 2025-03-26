use std::collections::HashSet;
use std::io::{self, BufRead};

fn max_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = *nums.iter().max().unwrap_or(&0);

    // Precompute factors for each number up to m
    let mut fac: Vec<Vec<i32>> = vec![Vec::new(); (m + 1) as usize];
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the nums vector
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the nums vector
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        nums.push(num);
    }

    // Compute the result
    let result = max_length(nums);

    // Print the result
    println!("{}", result);
}