use std::io;
use std::cmp;

// Helper function to return the maximum of two values
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Main logic implementation
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;                   // Tracks how many elements have been equal to `k` so far
    let mut f1 = vec![0; 51];         // Stores the maximum frequency for each number
    let mut max_f1 = 0;               // The maximum value in the `f1` array
    let mut f2 = 0;                   // Specific calculated result

    for &x in nums {
        // Update `f2` - considering the current element
        f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };

        // Update the frequency of the current number
        f1[x as usize] = max(f1[x as usize], f0) + 1;

        // If the current element equals `k`, update `f0`
        if x == k {
            f0 += 1;
        }

        // Update the maximum frequency in `f1`
        max_f1 = max(max_f1, f1[x as usize]);
    }

    // Return the final result
    max(max_f1, f2)
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the first line of input");
    let mut input_iter = input.trim().split_whitespace();
    
    // Parse `n` and `k`
    let n: usize = input_iter.next()
        .expect("Failed to parse n")
        .parse()
        .expect("Failed to convert n to usize");
    let k: i32 = input_iter.next()
        .expect("Failed to parse k")
        .parse()
        .expect("Failed to convert k to i32");

    // Read the array `nums`
    let mut nums = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Failed to read the second line of input");
    let nums: Vec<i32> = nums
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Failed to convert element to i32"))
        .collect();

    // Ensure the array size matches `n`
    if nums.len() != n {
        eprintln!("Error: Number of elements in nums does not match n");
        std::process::exit(1);
    }

    // Call the function and compute the result
    let result = max_frequency(&nums, k);

    // Output the result
    println!("{}", result);
}