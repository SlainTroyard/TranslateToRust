use std::io::{self, BufRead};

fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    // Helper function to check if a subarray is strictly increasing
    let mono = |idx: usize| -> bool {
        for i in idx..idx + k - 1 {
            if nums[i] >= nums[i + 1] {
                return false;
            }
        }
        true
    };

    // Iterate over possible starting indices of the first subarray
    for idx in 0..=nums.len() - 2 * k {
        if mono(idx) && mono(idx + k) {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array and the subarray length
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse n");
    
    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    // Read the length of the subarray
    let k: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse k");

    // Create a Solution object and call the function
    let result = has_increasing_subarrays(&nums, k);

    // Output the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}