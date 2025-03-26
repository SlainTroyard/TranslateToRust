use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Read the number of elements
    stdin.lock().read_line(&mut input).unwrap();
    let num_size: usize = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    // Read the elements into a vector
    let mut nums: Vec<usize> = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        stdin.lock().read_line(&mut input).unwrap();
        nums.push(input.trim().parse().expect("Please enter a valid number"));
        input.clear();
    }

    // Create an instance of Solution and get the result
    let solution = Solution;
    let result = solution.max_length(nums);

    // Print the result
    writeln!(stdout, "{}", result).unwrap();
}

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<usize>) -> usize {
        let n = nums.len();
        let m = *nums.iter().max().unwrap_or(&0);

        // Precompute the prime factors for each number up to m
        let mut fac: Vec<Vec<usize>> = vec![Vec::new(); m + 1];
        for i in 2..=m {
            if fac[i].is_empty() {
                for j in (i..=m).step_by(i) {
                    fac[j].push(i);
                }
            }
        }

        let mut ans = 2;
        let mut vis = vec![false; m + 1];

        let mut i = 0;
        let mut j = 0;
        while i < n {
            while j < n {
                let check = || {
                    for &p in &fac[nums[j]] {
                        if vis[p] {
                            return false;
                        }
                    }
                    true
                };

                if check() {
                    for &p in &fac[nums[j]] {
                        vis[p] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }

            ans = ans.max(j - i);
            for &p in &fac[nums[i]] {
                vis[p] = false;
            }
            i += 1;
        }

        ans
    }
}