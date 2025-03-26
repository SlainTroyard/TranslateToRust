use std::io::{self, Read};

/// Function to check if there are increasing subarrays
fn has_increasing_subarrays(nums: &[i32], nums_size: usize, k: usize) -> bool {
    if k == 1 {
        return true;  // Single element subarrays are always increasing
    }

    let mut a = 0;
    let mut s;
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
                return true;  // Found a valid increasing subarray
            }
        }
    }
    false  // No valid increasing subarray found
}

fn main() {
    // Create a string to store input
    let mut input = String::new();
    
    // Read the entire input
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut input_iter = input.split_whitespace();
    
    // Read the size of the array
    let n: usize = input_iter.next()
        .expect("Missing array size")
        .parse()
        .expect("Failed to parse array size");
    
    // Read the elements of the array
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = input_iter.next()
            .expect("Not enough array elements")
            .parse()
            .expect("Failed to parse array element");
        nums.push(num);
    }
    
    // Read the length of the subarray
    let k: usize = input_iter.next()
        .expect("Missing subarray length")
        .parse()
        .expect("Failed to parse subarray length");
    
    // Call the function to check for increasing subarrays
    if has_increasing_subarrays(&nums, n, k) {
        println!("true");
    } else {
        println!("false");
    }
}