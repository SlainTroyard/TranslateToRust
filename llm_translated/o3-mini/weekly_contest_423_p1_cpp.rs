use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Read the entire input from STDIN as a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input by whitespace. Note that the input can span multiple lines.
    let mut tokens = input.split_whitespace();
    
    // Parse the first token as the number of elements in the array.
    let n: usize = tokens
        .next()
        .expect("Expected a number for n")
        .parse()
        .expect("n should be a valid usize");
    
    // Read n numbers into the vector.
    let nums: Vec<i32> = (0..n)
        .map(|_| {
            tokens
                .next()
                .expect("Expected an integer for the array element")
                .parse()
                .expect("Array element should be a valid integer")
        })
        .collect();
    
    // Parse the next token as the length of the subarray, k.
    let k: usize = tokens
        .next()
        .expect("Expected a number for k")
        .parse()
        .expect("k should be a valid usize");
    
    // Check if there are two consecutive strictly increasing subarrays of length k.
    if has_increasing_subarrays(&nums, k) {
        println!("true");
    } else {
        println!("false");
    }
    
    Ok(())
}

// Function to check if there exist two consecutive strictly increasing subarrays of length k.
fn has_increasing_subarrays(nums: &Vec<i32>, k: usize) -> bool {
    // Ensure there are enough elements to form two subarrays.
    if nums.len() < 2 * k {
        return false;
    }
    
    // Closure to check if a subarray of length k starting at index 'idx' is strictly increasing.
    let mono = |idx: usize| -> bool {
        for i in idx..(idx + k - 1) {
            if nums[i] >= nums[i + 1] {
                return false;
            }
        }
        true
    };
    
    // Iterate over all valid starting indices.
    // The loop goes from idx = 0 to idx <= nums.len() - 2 * k.
    for idx in 0..=(nums.len() - 2 * k) {
        if mono(idx) && mono(idx + k) {
            return true;
        }
    }
    
    false
}