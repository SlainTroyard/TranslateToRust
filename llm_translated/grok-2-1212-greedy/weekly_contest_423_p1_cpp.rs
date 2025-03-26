use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
        let mono = |idx: usize| -> bool {
            for i in idx..idx + k - 1 {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        for idx in 0..=nums.len() - 2 * k {
            if mono(idx) && mono(idx + k) {
                return true;
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap()?.parse().unwrap();

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .by_ref()
        .take(n)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Read the length of the subarray
    let k: usize = lines.next().unwrap()?.parse().unwrap();

    // Call the function and print the result
    if Solution::has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}