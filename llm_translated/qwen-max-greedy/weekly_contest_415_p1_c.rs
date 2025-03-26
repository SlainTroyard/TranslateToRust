use std::io;

fn get_sneaky_numbers(nums: &[i32], return_size: &mut usize) -> Vec<i32> {
    let mut result = vec![0; 2];
    let mut count = 0;
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                result[count] = nums[i];
                count += 1;
                break;
            }
        }
        if count == 2 {
            break;
        }
    }
    *return_size = 2;
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let num_size: usize = input.trim().parse()?;
    
    // Allocate space for the array, adding 2 to the size as per the original code
    let mut nums = vec![0; num_size + 2];

    // Read the numbers into the array
    for i in 0..(num_size + 2) {
        input.clear(); // Clear the buffer
        io::stdin().read_line(&mut input)?;
        nums[i] = input.trim().parse()?;
    }

    // Get the sneaky numbers
    let mut return_size = 0;
    let result = get_sneaky_numbers(&nums, &mut return_size);

    // Print the result
    for i in 0..return_size {
        print!("{} ", result[i]);
    }
    println!();

    Ok(())
}