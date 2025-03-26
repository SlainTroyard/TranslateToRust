use std::io::{self, Read};

/// Finds the minimum positive sum of a subarray with length between l and r.
/// Returns -1 if no such subarray exists.
fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let nums_size = nums.len();
    // Use Option<i32> instead of '\0' character in C
    let mut min_sum: Option<i32> = None;
    
    for current_l in l..=r {
        let mut sum = 0;
        for i in 0..nums_size {
            sum += nums[i];
            if i >= current_l {
                sum -= nums[i - current_l];
            }
            if sum > 0 && i >= current_l - 1 {
                if min_sum.is_none() || min_sum.unwrap() > sum {
                    min_sum = Some(sum);
                }
            }
        }
    }
    
    min_sum.unwrap_or(-1)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut values = input.split_whitespace();
    
    // Input the size of the array
    let nums_size: usize = values.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to read array size"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid array size"))?;
    
    // Collect the array elements
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = values.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to read array element"))?
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid array element"))?;
        nums.push(num);
    }
    
    // Input the range [l, r]
    let l: usize = values.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to read l"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid l value"))?;
    
    let r: usize = values.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to read r"))?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid r value"))?;
    
    // Call the function
    let result = minimum_sum_subarray(&nums, l, r);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}