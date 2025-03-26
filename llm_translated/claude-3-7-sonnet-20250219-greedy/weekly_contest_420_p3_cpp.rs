use std::io::{self, BufRead};

const MX: usize = 1_000_001;

struct Solution;

impl Solution {
    fn min_operations(nums: &mut Vec<i32>) -> i32 {
        // Initialize the Least Prime Factor array
        let lpf = {
            let mut lpf = [0; MX];
            for i in 2..MX {
                if lpf[i] == 0 {
                    for j in (i..MX).step_by(i) {
                        if lpf[j] == 0 {
                            lpf[j] = i as i32;
                        }
                    }
                }
            }
            lpf
        };

        let mut ans = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                nums[i] = lpf[nums[i] as usize];
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure we have exactly n elements
    nums.truncate(n);
    
    // Solve the problem
    let solution = Solution::min_operations(&mut nums);
    
    // Print the result
    println!("{}", solution);
}