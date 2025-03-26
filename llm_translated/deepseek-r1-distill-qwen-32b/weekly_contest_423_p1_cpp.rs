use std::io;

struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
        let n = nums.len();
        if 2 * k > n {
            return false;
        }

        let mono = |idx: usize| -> bool {
            for i in idx..idx + k - 1 {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        for idx in 0..=(n - 2 * k) {
            if mono(idx) && mono(idx + k) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let n = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    let nums = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>()
    };

    let k = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    let sol = Solution;
    let result = sol.has_increasing_subarrays(&nums, k);
    println!("{}", result);
}