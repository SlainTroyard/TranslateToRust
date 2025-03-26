use std::io;

/// Checks if there are increasing subarrays of length `k` in the given array `nums`.
fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    if k == 1 {
        return true; // Single element subarrays are always increasing
    }

    let nums_size = nums.len();
    for _ in 0..2 {
        for i in 0..=nums_size - 2 * k {
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
    // Read the size of the array and the subarray length
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Read the elements of the array
    let mut nums = Vec::with_capacity(n);
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

    // Call the function to check for increasing subarrays
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}