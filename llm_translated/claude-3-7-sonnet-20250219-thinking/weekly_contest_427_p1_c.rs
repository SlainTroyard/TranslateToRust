use std::io::{self, Read};

/// Constructs a transformed array based on the given nums array.
/// For each element in nums, we calculate a new value based on specific rules.
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size];

    for i in 0..nums_size {
        if nums[i] == 0 {
            result[i] = nums[i];
        } else {
            let steps = nums[i];
            let mut target_index = (i as i32 + steps) % nums_size as i32;

            if target_index < 0 {
                target_index += nums_size as i32; // Handle negative wrapping
            }

            result[i] = nums[target_index as usize];
        }
    }

    result
}

fn main() -> io::Result<()> {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut iter = input.split_whitespace();
    
    // Parse the size of the array
    let nums_size: usize = iter.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing nums_size"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse nums_size"))?;
    
    // Parse the array elements
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = iter.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing number"))?
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse number"))?;
        nums.push(num);
    }

    // Transform the array
    let transformed_array = construct_transformed_array(&nums);

    // Print the result with the same format as the original C code
    for &num in &transformed_array {
        print!("{} ", num);
    }
    println!();

    Ok(())
}