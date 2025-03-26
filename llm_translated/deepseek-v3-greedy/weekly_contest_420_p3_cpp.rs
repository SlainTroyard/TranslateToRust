// Problem: Weekly Contest 420 Problem 3
use std::io::{self, BufRead};

const MX: usize = 1_000_001;
static mut LPF: [usize; MX] = [0; MX];

// Initialize the LPF array using a closure that runs at the start of the program
fn init_lpf() {
    unsafe {
        for i in 2..MX {
            if LPF[i] == 0 {
                for j in (i..MX).step_by(i) {
                    if LPF[j] == 0 {
                        LPF[j] = i;
                    }
                }
            }
        }
    }
}

struct Solution;

impl Solution {
    fn min_operations(nums: &mut Vec<usize>) -> i32 {
        let mut ans = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                unsafe {
                    nums[i] = LPF[nums[i]];
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
    init_lpf();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line which contains the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the second line which contains the array elements
    let mut nums: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Create an instance of Solution and call the min_operations method
    let sol = Solution;
    let result = sol.min_operations(&mut nums);

    // Print the result
    println!("{}", result);
}