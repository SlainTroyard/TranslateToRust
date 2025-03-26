use std::io::{self, BufRead};
use std::cmp;

fn min_operations(nums: &mut Vec<i32>) -> i32 {
    let nums_size = nums.len();
    let mut res = 0;
    let mut max_factor;
    let mut j;

    if nums_size == 1 {
        return res;
    }

    for i in (0..nums_size - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut max;
            let mut count = 1;
            let original = nums[i];
            
            loop {
                max = i32::MIN;
                max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                
                for factor in 2..=max_factor {
                    j = factor; // To match the original C code variable naming
                    if nums[i] % j == 0 {
                        max = cmp::max(max, j);
                        if nums[i] % (nums[i] / j) == 0 {
                            max = cmp::max(max, nums[i] / j);
                        }
                    }
                }
                
                if max == i32::MIN {
                    return -1;
                }
                
                count *= max;
                if count == original {
                    return -1;
                }
                
                nums[i] /= max;
                if nums[i] <= nums[i + 1] {
                    res += 1;
                    break;
                }
                
                res += 1; // Count each operation
            }
        }
    }

    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the array size
    let nums_size: usize = lines
        .next()
        .expect("Failed to read input")
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse array size");
    
    // Read the array elements
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = lines
            .next()
            .expect("Failed to read input")
            .expect("Failed to read input")
            .trim()
            .parse()
            .expect("Failed to parse array element");
        nums.push(num);
    }
    
    // Calculate and print the result
    println!("{}", min_operations(&mut nums));
    
    Ok(())
}