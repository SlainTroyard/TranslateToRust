use std::io;
use std::io::prelude::*;
use std::f64;
use std::cmp;

fn min_operations(nums: &mut Vec<i32>) -> i32 {
    let mut res = 0;
    let mut max_factor;
    let mut j;
    let nums_size = nums.len();

    if 1 == nums_size {
        return res;
    }

    for i in (0..nums_size - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut max = i32::MIN;
            let mut count = 1;
            let original = nums[i];
            loop {
                max = i32::MIN;
                max_factor = (f64::sqrt(nums[i] as f64) as i32) + 1;
                j = 2;
                while j <= max_factor {
                    if nums[i] % j == 0 {
                        max = cmp::max(max, j);
                        if nums[i] % (nums[i] / j) == 0 {
                            max = cmp::max(max, nums[i] / j);
                        }
                    }
                    j += 1;
                }
                if max == i32::MIN || (count * max) == original {
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
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: i32 = nums_size_str.trim().parse().expect("Invalid input");

    let mut nums_input_str = String::new();
    io::stdin().read_line(&mut nums_input_str).expect("Failed to read line");
    let nums_str_vec: Vec<&str> = nums_input_str.trim().split_whitespace().collect();
    let mut nums: Vec<i32> = Vec::new();
    for s in nums_str_vec {
        nums.push(s.parse().expect("Invalid number"));
    }

    if nums.len() != nums_size as usize {
        panic!("Input size does not match the number of elements provided");
    }

    println!("{}", min_operations(&mut nums));
}