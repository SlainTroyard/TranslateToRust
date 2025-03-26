use std::io::{self, BufRead};

/// Function to check if there are increasing subarrays
fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    if k == 1 {
        return true; // Single element subarrays are always increasing
    }

    let nums_size = nums.len();
    for _ in 0..2 {
        for i in 0..nums_size - 2 * k + 1 {
            let s = i + k;
            let mut a = 0;

            // Check each pair within the subarray for increasing order
            for z in 0..k - 1 {
                if nums[i + z] < nums[i + z + 1] && nums[s + z] < nums[s + z + 1] {
                    a += 1;
                }
            }

            if a == k - 1 {
                return true; // Found a valid increasing subarray
            }
        }
    }

    false // No valid increasing subarray found
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .expect("Expected input for array size")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse array size");

    // Read the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for array elements")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    assert_eq!(nums.len(), n, "Array size does not match the number of elements provided");

    // Read the length of the subarray
    let k: usize = lines
        .next()
        .expect("Expected input for subarray length")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse subarray length");

    // Call the function to check for increasing subarrays
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}