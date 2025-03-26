// Problem: Weekly Contest 420 Problem 3
use std::io::{self, BufRead};

const MX: usize = 1_000_001;
static mut LPF: [u32; MX] = [0; MX];

fn init() {
    unsafe {
        for i in 2..MX {
            if LPF[i] == 0 {
                for j in (i..MX).step_by(i) {
                    if LPF[j] == 0 {
                        LPF[j] = i as u32;
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
                    nums[i] = LPF[nums[i] as usize] as i32;
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

fn main() {
    // Initialize the LPF array
    init();

    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");
    let mut nums: Vec<i32> = vec![0; n];
    for i in 0..n {
        nums[i] = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse num");
    }

    // Solve the problem
    let sol = Solution;
    let result = sol.min_operations(&mut nums);

    // Output the result
    println!("{}", result);
}