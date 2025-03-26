use std::io;
use std::io::Read;
use std::cmp;
use std::num::ParseIntError;

fn min_operations(nums: &mut [i32]) -> i32 {
    let mut res = 0;
    let mut max_factor;
    let mut j;

    if nums.len() == 1 {
        return res;
    }

    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut max = i32::MIN;
            let mut count = 1;
            let original = nums[i];
            loop {
                max = i32::MIN;
                max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                j = 2;
                while j <= max_factor {
                    if nums[i] % j == 0 {
                        max = cmp::max(max, j);
                        if nums[i] / j != j && nums[i] % (nums[i] / j) == 0 {
                            max = cmp::max(max, nums[i] / j);
                        }
                    }
                    j += 1;
                }
                if max == i32::MIN || { count *= max; count } == original {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let nums_size: usize = lines
        .next()
        .ok_or("Expected numsSize")?
        .parse()?;

    let nums_str: Vec<&str> = lines
        .next()
        .ok_or("Expected nums line")?
        .split_whitespace()
        .collect();

    if nums_str.len() != nums_size {
        return Err("numsSize does not match number of elements".into());
    }

    let mut nums: Vec<i32> = nums_str
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;

    let result = min_operations(&mut nums);
    println!("{}", result);

    Ok(())
}