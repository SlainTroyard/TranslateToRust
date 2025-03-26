use std::io;

// Function to check if there are increasing subarrays
fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    if k == 1 {
        return true;  // Single element subarrays are always increasing
    }

    let n = nums.len();
    for j in 0..2 {
        for i in 0..n - 2 * k + 1 {
            let s = i + k;
            let mut a = 0;
            // Check each pair within the subarray for increasing order
            for z in 0..k - 1 {
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
    let mut input = String::new();

    // Read the size of the array
    println!("Enter the size of the array:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");
    input.clear();

    // Read the elements of the array
    let mut nums = Vec::with_capacity(n);
    println!("Enter the elements of the array:");
    for _ in 0..n {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        nums.push(input.trim().parse::<i32>().expect("Please type a number!"));
        input.clear();
    }

    // Read the length of the subarray
    println!("Enter the length of the subarray:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Please type a number!");

    // Call the function to check for increasing subarrays
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
}