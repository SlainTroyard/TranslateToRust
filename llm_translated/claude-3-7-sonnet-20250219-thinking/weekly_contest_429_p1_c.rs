use std::io::{self, Read};

/// Calculates the minimum operations required
/// This function analyzes an array of integers and returns the
/// minimum operations needed based on the problem constraints.
fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101]; // Assuming numbers are in the range 0-100
    
    // Iterate through array from right to left
    for i in (0..nums.len()).rev() {
        count[nums[i] as usize] += 1;
        // If we find a duplicate number
        if count[nums[i] as usize] > 1 {
            return ((i + 3) / 3) as i32;
        }
    }
    0
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut iter = input.split_whitespace();
    
    // Read the size of the array
    let n: usize = iter.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing array size"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid array size"))?;
    
    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = iter.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing array element"))?
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid array element"))?;
        nums.push(num);
    }
    
    // Calculate and print the result
    let result = minimum_operations(&nums);
    println!("{}", result);
    
    Ok(())
}