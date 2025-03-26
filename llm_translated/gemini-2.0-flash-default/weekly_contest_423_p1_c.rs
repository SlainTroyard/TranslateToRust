use std::io;

// Function to check if there are increasing subarrays
fn has_increasing_subarrays(nums: &[i32], k: i32) -> bool {
    let nums_size = nums.len();
    if k == 1 {
        return true; // Single element subarrays are always increasing
    }

    let k_usize = k as usize;
    for _j in 0..2 {
        for i in 0..(nums_size - 2 * k_usize + 1) {
            let s = i + k_usize;
            let mut a = 0;
            // Check each pair within the subarray for increasing order
            for z in 0..(k_usize - 1) {
                if nums[i + z] < nums[i + z + 1] && nums[s + z] < nums[s + z + 1] {
                    a += 1;
                }
            }
            if a == k_usize - 1 {
                return true; // Found a valid increasing subarray
            }
        }
    }
    false // No valid increasing subarray found
}

fn main() {
    let mut n_str = String::new();
    io::stdin()
        .read_line(&mut n_str)
        .expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin()
        .read_line(&mut nums_str)
        .expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    assert_eq!(nums.len(), n); // Check that the number of inputs matches the declared array size

    let mut k_str = String::new();
    io::stdin()
        .read_line(&mut k_str)
        .expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Invalid input");

    // Call the function to check for increasing subarrays
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}