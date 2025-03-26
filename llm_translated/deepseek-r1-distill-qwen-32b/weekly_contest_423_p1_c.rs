use std::io;

// Read a single line of input and trim whitespace
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Function to check if there are increasing subarrays
fn has_increasing_subarrays(nums: &[i32], k: i32) -> bool {
    if k == 1 {
        return true;  // Single element subarrays are always increasing
    }

    let n = nums.len() as i32;
    for _ in 0..2 {
        for i in 0..(n - 2 * k + 1) {
            let s = i + k;
            let mut a = 0;
            // Check each pair within the subarray for increasing order
            for z in 0..(k - 1) {
                let idx1 = i + z;
                let idx2 = i + z + 1;
                let idx3 = s + z;
                let idx4 = s + z + 1;
                if nums[idx1 as usize] < nums[idx2 as usize] && nums[idx3 as usize] < nums[idx4 as usize] {
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
    // Read the size of the array
    let n = read_input().parse::<i32>().expect("Invalid n");
    
    // Read the elements of the array
    let mut nums = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let num = read_input().parse::<i32>().expect("Invalid number");
        nums.push(num);
    }

    // Read the length of the subarray
    let k = read_input().parse::<i32>().expect("Invalid k");

    // Call the function to check for increasing subarrays
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}