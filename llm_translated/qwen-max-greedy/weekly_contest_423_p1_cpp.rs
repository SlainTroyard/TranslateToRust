use std::io;

fn main() {
    // Read the size of the array and the subarray length
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Read the elements of the array
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Read the length of the subarray
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Please type a number!");

    // Create a Solution object and call the function
    let sol = Solution;
    if sol.has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}

struct Solution;

impl Solution {
    fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
        // Helper closure to check if a subarray is strictly increasing
        let mono = |idx: usize| -> bool {
            for i in idx..idx + k - 1 {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            }
            true
        };

        // Iterate through the array to find two consecutive increasing subarrays of length k
        for idx in 0..=nums.len() - 2 * k {
            if mono(idx) && mono(idx + k) {
                return true;
            }
        }
        false
    }
}