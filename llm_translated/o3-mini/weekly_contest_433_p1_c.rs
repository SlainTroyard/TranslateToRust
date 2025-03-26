use std::io::{self, Read};

/// Computes the sum of all subarray differences based on prefix sums.
/// This function mirrors the logic of the provided C implementation.
///
/// The algorithm works by computing prefix sums in `sums` and then, for each index,
/// it adds the difference between the current prefix sum and the prefix sum at a
/// "valid" index determined using max(0, i - nums[i]).
/// 
/// # Arguments
///
/// * `nums` - A slice of i32 numbers.
///
/// # Returns
///
/// * The computed result as an i32.
fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    // Create a vector for prefix sums with n+1 elements (initialize all to 0)
    let mut sums = vec![0; n + 1];
    
    // Build prefix sums and update the answer.
    for i in 0..n {
        sums[i + 1] = nums[i] + sums[i];
        // Compute index = max(0, i - nums[i])
        // Since i is usize and nums[i] is i32, we cast i to i32 for subtraction.
        let index = if (i as i32 - nums[i]) > 0 { (i as i32 - nums[i]) as usize } else { 0 };
        ans += sums[i + 1] - sums[index];
    }
    
    ans
}

fn main() -> io::Result<()> {
    // Read the entire input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Split the input by whitespace to get tokens.
    let mut tokens = input.split_whitespace();
    
    // Read the first token as the number of elements.
    let n: usize = tokens
        .next()
        .expect("Expected an integer for n")
        .parse()
        .expect("Failed to parse n");
    
    // Read the next n tokens as integers.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens
            .next()
            .expect("Expected another integer")
            .parse()
            .expect("Failed to parse an integer");
        nums.push(num);
    }
    
    // Compute the result using the subarray_sum function.
    let result = subarray_sum(&nums);
    
    // Print the result in the exact same format as the original C code.
    println!("{}", result);
    
    Ok(())
}