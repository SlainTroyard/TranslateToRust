// Problem: Weekly Contest 420 Problem 3

use std::io::{self, BufRead};

const MX: usize = 1_000_001;
static mut LPF: [i32; MX] = [0; MX];

// Initialize LPF array
fn init_lpf() {
    for i in 2..MX {
        unsafe {
            if LPF[i] == 0 {
                for j in (i..MX).step_by(i) {
                    if LPF[j] == 0 {
                        LPF[j] = i as i32;
                    }
                }
            }
        }
    }
}

struct Solution;

impl Solution {
    fn min_operations(nums: &mut Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                unsafe {
                    nums[i] = LPF[nums[i] as usize];
                }
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Initialize LPF array
    init_lpf();

    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read nums
    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Process and output result
    let result = Solution::min_operations(&mut nums);
    println!("{}", result);

    Ok(())
}