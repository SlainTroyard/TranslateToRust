use std::io::{self, BufRead};

/// Computes the maximum "good number" based on given array
fn max_good_number(nums: &[i32]) -> i32 {
    let mut nums1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;
    
    // Count consecutive 0 bits in 7th position for first three elements
    for i in 0..nums.len() {
        for j in 0..7 {
            if i == 0 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    nums1 += 1;
                } else {
                    break;
                }
            } else if i == 1 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    num2 += 1;
                } else {
                    break;
                }
            } else if i == 2 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    num3 += 1;
                } else {
                    break;
                }
            }
        }
    }
    
    // Store counts and indices
    let times = vec![nums1, num2, num3];
    let mut store = vec![0, 1, 2];
    
    // Sort the elements based on custom comparison
    for i in 1..=2 {
        for j in 0..=1 {
            let value1 = (nums[store[j]] << (7 - times[store[j + 1]])) + nums[store[j + 1]];
            let value2 = (nums[store[j + 1]] << (7 - times[store[j]])) + nums[store[j]];
            
            if value2 >= value1 {
                // Swap elements
                let temp = store[j];
                store[j] = store[j + 1];
                store[j + 1] = temp;
            }
        }
    }
    
    // Calculate final result
    (nums[store[0]] << (14 - times[store[1]] - times[store[2]])) +
    (nums[store[1]] << (7 - times[store[2]])) +
    nums[store[2]]
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    // Read number of elements
    let line = iterator.next().unwrap()?;
    let num_size: usize = line.trim().parse().expect("Invalid input");
    
    // Read array elements
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let line = iterator.next().unwrap()?;
        nums.push(line.trim().parse::<i32>().expect("Invalid integer"));
    }
    
    // Compute and print result
    println!("{}", max_good_number(&nums));
    
    Ok(())
}