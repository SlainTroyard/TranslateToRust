use std::io;
use std::io::Read;

// Function to construct the transformed array
fn construct_transformed_array(nums: &[i32]) -> Vec<i32> {
    let nums_size = nums.len();
    let mut result = vec![0; nums_size];

    for i in 0..nums_size {
        if nums[i] == 0 {
            result[i] = nums[i];
        } else {
            let steps = nums[i];
            let target_index = (i as i32 + steps) % nums_size as i32;

            let target_index = if target_index < 0 {
                (target_index + nums_size as i32) as usize
            } else {
                target_index as usize
            };

            result[i] = nums[target_index];
        }
    }

    result
}

// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let nums_size: usize = lines.next().unwrap().parse()?;

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let transformed_array = construct_transformed_array(&nums);

    for &val in &transformed_array {
        print!("{} ", val);
    }
    println!();

    Ok(())
}