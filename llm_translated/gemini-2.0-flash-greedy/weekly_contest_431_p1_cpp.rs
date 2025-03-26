use std::io;
use std::io::Read;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_length(&self, nums: &Vec<i32>) -> i32 {
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
        let mut vis: Vec<bool> = vec![false; (m + 1) as usize];
        let mut j = 0;
        for i in 0..n {
            while j < n {
                let check = || {
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
    let mut num_size_str = String::new();
    io::stdin().read_line(&mut num_size_str).expect("Failed to read line");
    let num_size: i32 = num_size_str.trim().parse().expect("Invalid input");

    let mut nums: Vec<i32> = Vec::new();
    for _ in 0..num_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    let solution = Solution {};
    let result = solution.max_length(&nums);
    println!("{}", result);
}