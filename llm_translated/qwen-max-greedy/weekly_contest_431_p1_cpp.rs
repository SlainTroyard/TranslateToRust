use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut nums: Vec<i32> = Vec::new();

    // Read the size of the array
    let num_size: usize = {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    // Read the array elements
    for _ in 0..num_size {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        nums.push(input.trim().parse().unwrap());
    }

    // Create a solution instance and solve the problem
    let solution = Solution;
    let result = solution.max_length(nums);

    // Output the result
    writeln!(stdout, "{}", result).unwrap();
}

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = *nums.iter().max().unwrap_or(&0);

        // Precompute the prime factors for each number up to m
        let mut fac: Vec<Vec<i32>> = vec![Vec::new(); (m + 1) as usize];
        for i in 2..=m {
            if fac[i as usize].is_empty() {
                for j in (i..=m).step_by(i as usize) {
                    fac[j as usize].push(i);
                }
            }
        }

        let mut ans = 2;
        let mut vis: Vec<bool> = vec![false; (m + 1) as usize];

        let mut i = 0;
        let mut j = 0;
        while i < n {
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
            i += 1;
        }

        ans
    }
}