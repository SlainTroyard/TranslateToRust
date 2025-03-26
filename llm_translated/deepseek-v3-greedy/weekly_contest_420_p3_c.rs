use std::io::{self, BufRead};
use std::cmp;

fn min_operations(nums: &mut Vec<i32>) -> i32 {
    let mut res = 0;
    let mut max_factor = 0;
    let mut j = 0;

    if nums.len() == 1 {
        return res;
    }

    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let original = nums[i];
            let mut count = 1;
            loop {
                let mut max = i32::MIN;
                max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        max = cmp::max(max, j);
                        if nums[i] % (nums[i] / j) == 0 {
                            max = cmp::max(max, nums[i] / j);
                        }
                    }
                }
                if max == i32::MIN || (count *= max) == original {
                    return -1;
                } else {
                    nums[i] /= max;
                    if nums[i] <= nums[i + 1] {
                        res += 1;
                        break;
                    }
                }
            }
        }
    }

    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line to get the size of the array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the second line to get the array elements
    let nums_line = lines.next().unwrap().unwrap();
    let mut nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure the number of elements matches the expected size
    assert_eq!(nums.len(), nums_size);
    
    // Compute the result
    let result = min_operations(&mut nums);
    
    // Print the result
    println!("{}", result);
}