use std::io;

fn has_increasing_subarrays(nums: &[i32], nums_size: usize, k: usize) -> bool {
    if k == 1 {
        return true; // Single element subarrays are always increasing
    }

    let mut a = 0;
    let mut s: usize;
    for j in 0..2 {
        for i in 0..(nums_size - 2 * k + 1) {
            s = i + k;
            a = 0;
            // Check each pair within the subarray for increasing order
            for z in 0..(k - 1) {
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
    let mut input = String::new();
    let stdin = io::stdin();
    
    // Read number of elements in the array
    stdin.read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input for n");
    input.clear();

    // Read array elements
    let mut nums: Vec<i32> = Vec::new();
    stdin.read_line(&mut input).expect("Failed to read input");
    nums.extend(
        input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid input for array elements")),
    );
    assert!(nums.len() == n, "Array length mismatch");
    input.clear();

    // Read the subarray length
    stdin.read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Invalid input for k");

    // Call the function to check for increasing subarrays
    if has_increasing_subarrays(&nums, n, k) {
        println!("true");
    } else {
        println!("false");
    }
}