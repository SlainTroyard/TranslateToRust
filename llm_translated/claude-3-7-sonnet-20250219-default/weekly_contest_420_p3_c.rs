use std::io::{self, BufRead, BufReader};
use std::cmp::{max, min};

/// Minimum operations to make array decreasing (LeetCode Weekly Contest 420 Problem 3)
/// 
/// This function calculates the minimum number of operations needed to make an array
/// non-increasing (monotonically decreasing). In each operation, we can divide an element
/// by one of its factors (except 1).
fn min_operations(nums: &mut [i32]) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;
    
    // If there's only one element, no operations needed
    if nums_size == 1 {
        return res;
    }
    
    // Start from the second-to-last element and work backwards
    for i in (0..nums_size-1).rev() {
        // If current element is greater than the next one, we need to reduce it
        if nums[i] > nums[i + 1] {
            let mut max_val;
            let mut count = 1;
            let original = nums[i];
            
            loop {
                max_val = i32::MIN;
                // Only need to check factors up to sqrt(nums[i])
                let max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                
                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        // Found a factor
                        max_val = max(max_val, j);
                        // Check the corresponding larger factor (nums[i] / j)
                        if nums[i] % (nums[i] / j) == 0 {
                            max_val = max(max_val, nums[i] / j);
                        }
                    }
                }
                
                // If no factors found or operation would be cyclic, return -1
                if max_val == i32::MIN || (count *= max_val) == original {
                    return -1;
                } else {
                    // Divide by the largest factor
                    nums[i] /= max_val;
                    res += 1;
                    
                    // If array is now non-increasing at this position, we're done with this element
                    if nums[i] <= nums[i + 1] {
                        break;
                    }
                }
            }
        }
    }
    
    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    
    // Read the array size
    let mut size_input = String::new();
    reader.read_line(&mut size_input)?;
    let nums_size: usize = size_input.trim().parse().expect("Expected a valid integer");
    
    // Read the array elements
    let mut elements_input = String::new();
    reader.read_line(&mut elements_input)?;
    let mut nums: Vec<i32> = elements_input
        .split_whitespace()
        .map(|s| s.parse().expect("Expected a valid integer"))
        .collect();
    
    // Ensure we have the correct number of elements
    nums.truncate(nums_size);
    
    // Calculate and print the result
    println!("{}", min_operations(&mut nums));
    
    Ok(())
}